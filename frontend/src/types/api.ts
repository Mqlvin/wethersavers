export class DirectRequest<T> {
    success: boolean;
    error_reason?: string;
    data?: T;

    // we access raw string (req response) in
    constructor(jsonText: string) {
        let parsed: any;

        // parse as json
        try {
            parsed = JSON.parse(jsonText);
        } catch(e) {
            this.success = false;
            this.error_reason = "Failed to parse JSON from server";
            this.data = undefined;
            return;
        }

        this.success = parsed.success;
        this.error_reason = parsed.error_reason;
        this.data = parsed.data as T | undefined;
    }

    isOk(): boolean {
        return this.success && !this.error_reason;
    }

    isErr(): boolean {
        return !this.success || !!this.error_reason;
    }
}

