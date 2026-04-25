use std::{sync::Arc, time::{Duration, SystemTime}};

use dashmap::DashMap;
use once_cell::sync::Lazy;

use crate::wetherspoons::{drinks::*, error::WetherspoonsError};

const CACHE_LENGTH_MINS: u64 = 60;

struct CacheEntry {
    value: Arc<Vec<Drink>>,
    next_refresh: SystemTime,
}

// venue identifier, drinks menu
static DRINKS_CACHE: Lazy<DashMap<usize, CacheEntry>> = Lazy::new(|| {
    DashMap::new()
});

// either 1 hour from now, or 9am british time (whichever is sooner)
fn compute_next_refresh(last_fetched: SystemTime) -> SystemTime {
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
            let next_refresh = compute_next_refresh(now);

            DRINKS_CACHE.insert(*venue_identifier, CacheEntry { value: arc_vec.clone(), next_refresh });
            Ok(arc_vec)
        },
        Err(err) => {
            Err(WetherspoonsError::ParseDrinksError(format!("Error in cache mechanism: {}", err.to_string())))
        }
    }
}
