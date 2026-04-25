use std::{sync::{Arc}, time::{Duration, SystemTime}};

use dashmap::DashMap;
use flate2::{Compression, bufread::GzEncoder};
use once_cell::{sync::Lazy};
use tokio::sync::RwLock;

use crate::wetherspoons::{drinks::*, error::WetherspoonsError, get_venues};

const CACHE_LENGTH_MINS: u64 = 60;

/*
* DRINKS CACHE 
*/
struct DrinksCacheEntry {
    value: Arc<Vec<Drink>>,
    next_refresh: SystemTime,
}

// venue identifier, drinks menu
static DRINKS_CACHE: Lazy<DashMap<usize, DrinksCacheEntry>> = Lazy::new(|| {
    DashMap::new()
});

// either 1 hour from now, or 9am british time (whichever is sooner)
fn compute_next_drinks_refresh(last_fetched: SystemTime) -> SystemTime {
    let next_by_cache_life = last_fetched + Duration::from_mins(CACHE_LENGTH_MINS);
    next_by_cache_life
}

pub fn has_valid_drinks_cache(venue_identifier: &usize) -> bool {
    let now = SystemTime::now();

    if let Some(entry) = DRINKS_CACHE.get(venue_identifier) {
        if now < entry.next_refresh {
            return true;
        }
    }
    false
}

pub async fn get_cached_drinks_menu(venue_identifier: &usize, sales_id: &usize, drinks_menu_id: &usize) -> Result<Arc<Vec<Drink>>, WetherspoonsError> {
    let now = SystemTime::now();

    if let Some(entry) = DRINKS_CACHE.get(venue_identifier) {
        if now < entry.next_refresh {
            return Ok(entry.value.clone()); // dont need to refresh
        }
    }

    let drinks_fetch = get_drinks_menu(venue_identifier, sales_id, drinks_menu_id).await;
    match drinks_fetch {
        Ok(drinks) => {
            let arc_vec = Arc::new(drinks);
            let next_refresh = compute_next_venue_refresh(now);

            DRINKS_CACHE.insert(*venue_identifier, DrinksCacheEntry { value: arc_vec.clone(), next_refresh });
            Ok(arc_vec)
        },
        Err(err) => {
            Err(WetherspoonsError::ParseDrinksError(format!("Error in cache mechanism: {}", err.to_string())))
        }
    }
}



/*
* VENUES CACHE
*/
static VENUE_CACHE_ENCODED: Lazy<RwLock<Option<VenueCacheEntry>>> = Lazy::new(|| RwLock::new(None));

struct VenueCacheEntry {
    value: Arc<String>,
    next_refresh: SystemTime,
}

fn compute_next_venue_refresh(last_fetched: SystemTime) -> SystemTime {
    let next_by_cache_life = last_fetched + Duration::from_mins(CACHE_LENGTH_MINS);
    next_by_cache_life
}

pub async fn get_cached_venues_encoded() -> Result<Arc<String>, WetherspoonsError> {
    let now = SystemTime::now();

    {
        let guard = VENUE_CACHE_ENCODED.read().await;
        if let Some(entry) = guard.as_ref() {
            if now < entry.next_refresh {
                return Ok(Arc::clone(&entry.value));
            }
        }
        // guard dropped at end of this block
    }

    let serialized = match get_venues().await {
        Ok(str) => str,
        Err(err) => { return Err(err); }
    };
    let arc = Arc::new(serialized);
    let next_refresh = compute_next_venue_refresh(now);

    let mut write_guard = VENUE_CACHE_ENCODED.write().await;
    *write_guard = Some(VenueCacheEntry { value: Arc::clone(&arc), next_refresh });
    // write_guard dropped

    Ok(arc)
}
