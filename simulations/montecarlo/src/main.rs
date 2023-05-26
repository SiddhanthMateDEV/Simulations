/*Creating a basic template of the monte carlo simulation
Will tweak and modify this template when the code is done 
Will also add jump diffusion models to this 
 */


//  Pull the crates into the program 
// extern crate rand;
// extern crate rayon;
// extern crate ndarray;
// extern crate rusty_machine;
// extern crate serde;
// extern crate serde_json;

// Import the necessary dependencies
use rand::distributions::{Normal,Distribution};
use rand::Rng;
use rayon::prelude::*;
use rusty_machine::linalg::Matrix;
use serde::{Deserialize,Serialize}



// CREATE A STRUCT FOR INPUT PARAMETERS OF THE OPTION 

pub struct Option{
    S_O: f64,
    K: f64,
    R: f64,
    sigma: f64,
    T: f64,
    simulations: usize,
}

impl Option{
    // initialize all the variables into this from the relating structure
    pub fn new(S_O:f64,K:f64,R:f64,sigma:f64,T:f64) -> Self{
        Self{S_O,K,R,sigma,T}
    }

    
    /* below three functions are for price at expiry based on the current conditions 
     also the payoffs for calls and puts*/
    // -----------------------------------------------------------------------
    fn simulate_expiry_price(&self,z: f64) -> f64{
        let mut rng=rand::thread_rng();
        let z: f64=Normal::new(0.0,1.0).unwrap().sample(&mut rng);
        self.S_O*(-self.R*self.T+self.sigma*(self.T as f64).sqrt()*z).exp()
    }

    fn calculate_call_payoff(&self,expiry_price: f64) -> f64 {
        (expiry_price-self.K).max(0.0)
    }

    fn calculate_put_payoff(&self,expiry_price: f64) -> f64 {
        (expiry_price-self.K).max(0.0)
    }
    // -------------------------------------------------------------------------

    /* Below is the function for the Monte Carlo Simulation for n_Steps
     Going to use anithetic and control variates to reduce the computation requirements
     will later compare and show the difference between a brute force simulation and using 
     variates*/
    //  -------------------------------------------------------------------------

    fn monte_carlo_simulation(&self,n_simulations:usize) -> (f64,f64) {
        let mut rng=rand::thread_rng();
        let z_dist=Normal::new(0.0,1.0).unwrap();

        let mut total_call_payoff=0.0;
        let mut total_put_payoff=0.0;

        let mut total_control_variate_payoff=0.0;

        for _ in 0..n_simulations {
            let z=z_dist.sample(&mut rng);

            // orginal GBM Motion without variates in the model
            let expiry_price=self.simulate_expiry_price(z);
            total_call_payoffs += self.calculate_call_payoffs(expiry_price);
            total_put_payoffs += self.calculate_put_payoffs(expiry_price);

            /*  anithetic path for the model to create a negative dependency to 
             improve performance and also the solution*/
            let expiry_price_ant=self.simulate_expiry_price(-z);
            total_call_payoffs += self.call_payoffs(expiry_price_ant);
            total_put_payoffs += self.put_payoffs(expiry_price_ant);

            // control variate will be the average of the two paths for n steps
            total_control_variate+=expiry_price+expiry_price_ant;
        }

        let average_call_payoffs = total_call_payoffs/(2*n_simulations as f64);
        let average_put_payoffs = total_put_payoffs/(2*n_simulations as f64);

        let average_control_variate=total_control_variate/(n_simulations as f64);

        let call_payoff=average_call_payoffs-(average_control_variate-self.S_O)*self.calculate_call_payoff(self.S_0);
        let put_payoff=average_put_payoffs-(average_control_variate-self.S_O)*self.calculate_put_payoff(self.S_0);

        (call_payoff, put_payoff)
    // --------------------------------------------------------------------------
    }
}



fn main() {

    let option = Option::new(50000.0, 51000.0, 0.05, 0.1, 1.0);
    
    // let end_price = option.simulate_expiry_price();
    // let call_payoff = option.calculate_call_payoff(expiry_price);
    // let put_payoff = option.calculate_put_payoff(expiry_price);

    let n_simulations = 100000;
    let (estimated_call_payoff,estimated_put_payoff)=option.monte_carlo_simulation(n_simulations);
}



