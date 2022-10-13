/* 
 * The logic portion of our program.
 * 
 * Created by Philip Otter
 * October 2022
 */

 pub mod logic{

    
    pub fn analyze_input(input){
        // In the future I would like for the user to be able to adjust confidence rating.
        let confidence_rating = 0;

        let current_input: Vec<_> = input;  // Current input is a vector so we can test multiple decoded strings after first pass if confidence_rating is below a 5.

        // Sanity Checks
        println!("{}",("\nfn analze_input started\n").green());
        println!("{}",("Passed input:  ").cyan(),input.blue());
        println!("{}{:?}",("var confidence_rating set to:  ").cyan(),confidence_rating.blue());
        println!("{}{:?}",("var current_input:  ").cyan(), current_input)

        while confidence_rating < 5{
            let mut x = 0;
            let numeric: Bool = numeric_only(current_input[x]);
            for chars in current_input[x].chars(){
                let coded_chars: Vec<_> = chars;
            }

            println!("\n{}{:?}\n",("Vector coded_chars:  ").cyan(),coded_chars);  // Sanity Check

            x += 1;
        }
    }


    fn numeric_only(possibly_numeric_chars){
        for chars in possibly_numeric_chars{
            if !chars.is_numeric(){
                println!("{}{}{}",("char:  ").cyan(),chars.yellow(),(" is not numeric.").cyan());
                return False;
            }
            else{
                println!("{}{}{}",("char:  ").cyan(),chars.blue(),(" is numeric.").cyan());
            }
        }
        return True;
    }
 }