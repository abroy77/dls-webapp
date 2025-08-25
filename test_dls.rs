// Simple test to verify DLS calculation
mod dls;

fn main() {
    // Test case: Team 1 scored 250 in 50 overs
    // Team 2 played 20 overs, lost 3 wickets, match reduced to 40 overs
    let result = dls::get_target_score_simple(250, 20, 3, 40);
    
    match result {
        Ok(target) => println!("Target: {}", target),
        Err(e) => println!("Error: {}", e),
    }
    
    // Test case: Team 1 scored 180 in 50 overs  
    // Team 2 played 10 overs, lost 1 wicket, match reduced to 35 overs
    let result2 = dls::get_target_score_simple(180, 10, 1, 35);
    
    match result2 {
        Ok(target) => println!("Target 2: {}", target),
        Err(e) => println!("Error 2: {}", e),
    }
}
