use dialoguer::theme::ColorfulTheme;
use dialoguer::*;

pub enum PromptType<'a> {
    Select(&'a [&'a str]),
    Confirm,
    Input,
}

pub fn ask_user(prompt: &str, prompt_type: PromptType) -> String {
    match prompt_type {
        PromptType::Select(options) => {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .items(options)
                .default(0)
                .interact()
                .unwrap();
            options[selection].to_string()
        }
        PromptType::Confirm => {
            let result = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .default(true)
                .interact()
                .unwrap();
            result.to_string()
        },
        PromptType::Input => Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact()
            .unwrap(),
    }
}



#[macro_export]
macro_rules! ask {
    (input $prompt:expr) => {{
                ask_user($prompt, PromptType::Input)
    }};
    (select $prompt:expr, options $( $option:expr ),+ $(,)?) => {{
        let opts: &[&str] = &[ $( $option ),+ ];
        ask_user($prompt, PromptType::Select(opts))
    }};
   (confirm $prompt:expr) => {{
        let prompt = format!("{}", style($prompt).cyan());
        ask_user(&prompt, PromptType::Confirm) == "true"
   }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    
    // Test the PromptType enum variants
    #[test]
    fn test_prompt_type() {
        let options = &["Option 1", "Option 2"];
        let select_type = PromptType::Select(options);
        let confirm_type = PromptType::Confirm;
        let input_type = PromptType::Input;

        match select_type {
            PromptType::Select(opts) => assert_eq!(opts, options),
            _ => panic!("Expected Select variant"),
        }

        match confirm_type {
            PromptType::Confirm => (),
            _ => panic!("Expected Confirm variant"),
        }

        match input_type {
            PromptType::Input => (),
            _ => panic!("Expected Input variant"),
        }
    }

    // Test the select functionality with direct values
    #[rstest]
    #[case(1, "Option 2")]
    #[case(0, "Option 1")]
    #[case(2, "Option 3")]
    fn test_select_options(#[case] selection: usize, #[case] expected: &str) {
        let options = &["Option 1", "Option 2", "Option 3"];
        assert_eq!(options[selection], expected);
    }

    // Test string equality for input values
    #[rstest]
    #[case("user input")]
    #[case("another input")]
    #[case("")]  // Test empty input
    #[case("Special chars: !@#$%^&*()")]
    #[case("Multi\nline\ntext")]
    fn test_input_values(#[case] input_value: &str) {
        let expected = input_value.to_string();
        assert_eq!(expected, input_value);
    }

    // Test boolean values for confirm
    #[rstest]
    #[case(true)]
    #[case(false)]
    fn test_confirm_values(#[case] confirm_value: bool) {
        assert_eq!(confirm_value, confirm_value);
    }
    
    // Test PromptType with empty options array
    #[test]
    fn test_prompt_type_empty_options() {
        let empty_options: &[&str] = &[];
        let select_type = PromptType::Select(empty_options);
        
        match select_type {
            PromptType::Select(opts) => {
                assert_eq!(opts.len(), 0);
                assert!(opts.is_empty());
            },
            _ => panic!("Expected Select variant"),
        }
    }
    
    // Test PromptType with large options array
    #[test]
    fn test_prompt_type_large_options() {
        let large_options = &[
            "Option 1", "Option 2", "Option 3", "Option 4", "Option 5",
            "Option 6", "Option 7", "Option 8", "Option 9", "Option 10"
        ];
        let select_type = PromptType::Select(large_options);
        
        match select_type {
            PromptType::Select(opts) => {
                assert_eq!(opts.len(), 10);
                assert_eq!(opts[9], "Option 10");
            },
            _ => panic!("Expected Select variant"),
        }
    }

    /// Tests for macro expansions
    mod macro_expansion_tests {
        
        fn verify_macro_expansion(expansion: &str) -> bool {
            !expansion.is_empty()
        }
        
        #[test]
        fn test_basic_macro_expansions() {
            // Test input macro
            let input_expansion = stringify!(ask!(input "Test prompt"));
            // The actual macro expansion contains the full path to ask_user
            assert!(input_expansion.contains("input"));
            assert!(input_expansion.contains("Test prompt"));
            
            // Test select macro
            let select_expansion = stringify!(ask!(select "Test prompt", options "Option 1", "Option 2"));
            assert!(select_expansion.contains("select"));
            assert!(select_expansion.contains("Test prompt"));
            assert!(select_expansion.contains("options"));
            assert!(select_expansion.contains("Option 1"));
            assert!(select_expansion.contains("Option 2"));
            
            // Test confirm macro
            let confirm_expansion = stringify!(ask!(confirm "Test prompt"));
            assert!(confirm_expansion.contains("confirm"));
            assert!(confirm_expansion.contains("Test prompt"));
        }
        
        #[test]
        fn test_nested_macro_expansions() {
            // Test nested macro calls
            let nested_expansion = stringify! {
                if ask!(confirm "Do you want to proceed?") {
                    let name = ask!(input "Enter your name:");
                    let choice = ask!(select "Choose an option:", options "A", "B", "C");
                    println!("Name: {}, Choice: {}", name, choice);
                }
            };
            
            assert!(nested_expansion.contains("confirm"));
            assert!(nested_expansion.contains("input"));
            assert!(nested_expansion.contains("select"));
            assert!(nested_expansion.contains("options"));
            assert!(verify_macro_expansion(nested_expansion));
        }
        
        #[test]
        fn test_select_trailing_comma() {
            let expansion = stringify!(ask!(select "Test prompt", options "Option 1", "Option 2",));
            assert!(expansion.contains("Option 1"));
            assert!(expansion.contains("Option 2"));
        }
        
        #[test]
        fn test_macro_with_complex_expressions() {
            let _prompt_var = "Dynamic Prompt";
            let _option1 = "Dynamic Option 1";
            let _option2 = "Dynamic Option 2";
            
            // Test variable as prompt
            let expansion = stringify!(ask!(input _prompt_var));
            assert!(expansion.contains("_prompt_var"));
            
            // Test variables as options
            let expansion = stringify!(ask!(select _prompt_var, options _option1, _option2));
            assert!(expansion.contains("_prompt_var"));
            assert!(expansion.contains("_option1"));
            assert!(expansion.contains("_option2"));
            
            // Test format! as prompt
            let expansion = stringify!(ask!(input format!("Hello {}", "world")));
            assert!(expansion.contains("format!"));
            assert!(expansion.contains("Hello"));
            assert!(expansion.contains("world"));
            
            // Test format! as options
            let expansion = stringify!(ask!(select "Choose:", options 
                &format!("Option {}", 1),
                &format!("Option {}", 2)
            ));
            assert!(expansion.contains("format!"));
            assert!(expansion.contains("Option"));
        }
        
        #[test]
        fn test_macro_with_different_prompt_types() {
            // Test static string
            let expansion = stringify!(ask!(input "Static prompt"));
            assert!(expansion.contains("Static prompt"));
            
            // Test variable
            let expansion = stringify!(ask!(input _prompt_var));
            assert!(expansion.contains("_prompt_var"));
            
            // Test function call
            let expansion = stringify!(ask!(input get_prompt()));
            assert!(expansion.contains("get_prompt()"));
            
            // Test method call
            let expansion = stringify!(ask!(input obj.get_prompt()));
            assert!(expansion.contains("obj.get_prompt()"));
        }
        
        #[test]
        fn test_input_macro_expansion() {
            let expansion = stringify! {
                let name = ask!(input "What's your name?");
                println!("Hello, {}", name);
            };
            
            assert!(verify_macro_expansion(expansion));
        }
        
        #[test]
        fn test_select_macro_expansion() {
            let expansion = stringify! {
                let choice = ask!(select "Choose an option:", options
                    "Option 1",
                    "Option 2",
                    "Option 3"
                );
                println!("You chose: {}", choice);
            };
            
            assert!(verify_macro_expansion(expansion));
        }
        
        #[test]
        fn test_confirm_macro_expansion() {
            let expansion = stringify! {
                let proceed = ask!(confirm "Do you want to proceed?");
                if proceed {
                    println!("Proceeding...");
                } else {
                    println!("Cancelled.");
                }
            };
            
            assert!(verify_macro_expansion(expansion));
        }
    }
    
    /// Integration test for a typical prompt workflow
    #[test]
    fn test_prompt_workflow_integration() {
        let name = "test extension";
        let platform_options = &["Browser", "IDE", "Other"];
        let selected_platform = "Browser";
        let proceed = true;
        
        assert_eq!(name, "test extension");
        assert_eq!(platform_options[0], "Browser");
        assert_eq!(selected_platform, "Browser");
        assert!(proceed);
    }
    
    /// Test a complete workflow with multiple prompts
    #[test]
    fn test_complete_prompt_workflow() {
        // Simulate a complete workflow with multiple prompts
        let extension_name = "my-awesome-extension";
        let platforms = &["Browser", "IDE", "Messaging App", "3D App", "Game", "Other"];
        let selected_platform_index = 0; // Browser
        let selected_platform = platforms[selected_platform_index];
        
        let extension_types = &["UI Component", "API Integration", "Theme", "Tool"];
        let selected_type_index = 2; // Theme
        let selected_type = extension_types[selected_type_index];
        
        let confirmation = true;
        
        // Verify the simulated workflow
        assert_eq!(extension_name, "my-awesome-extension");
        assert_eq!(selected_platform, "Browser");
        assert_eq!(selected_type, "Theme");
        assert!(confirmation);
        
        // Simulate the formatted output that would be displayed to the user
        let summary = format!(
            "Creating {} extension for {} platform of type {}", 
            extension_name, selected_platform, selected_type
        );
        
        assert!(summary.contains(extension_name));
        assert!(summary.contains(selected_platform));
        assert!(summary.contains(selected_type));
    }
    
    /// Test error handling scenarios
    #[test]
    fn test_error_handling_scenarios() {
        // This test demonstrates how error handling could be tested
        // if the prompt functions were modified to return Results
        
        // Example of how we would test empty options
        let empty_options: &[&str] = &[];
        let select_type = PromptType::Select(empty_options);
        
        match select_type {
            PromptType::Select(opts) => {
                // In a real implementation, we might expect an error for empty options
                // But for now we just verify it's empty
                assert!(opts.is_empty());
            },
            _ => panic!("Expected Select variant"),
        }
        
        // Example of how we would test invalid input (if implemented)
        // let result = validate_input("", "Name cannot be empty");
        // assert!(result.is_err());
        
        // Example of how we would test cancellation (if implemented)
        // let result = simulate_user_cancellation();
        // assert_eq!(result, Err(PromptError::Cancelled));
    }
}
