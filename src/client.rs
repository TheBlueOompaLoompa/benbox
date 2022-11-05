use postgrest::Postgrest;

pub fn create_client() -> Postgrest {
    Postgrest::new("https://eblordkfqsqlklfpxnqn.supabase.co/rest/v1/")
        .insert_header("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImVibG9yZGtmcXNxbGtsZnB4bnFuIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NjY3NDcyMjIsImV4cCI6MTk4MjMyMzIyMn0.oJ6QbwNioZK-ZJSfLRH4Y-MHCT7jNvwiEe3jhA32GIA")
}