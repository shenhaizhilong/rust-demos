#[cfg(test)]
pub mod macros_demo {
    /**
    macro_rules! $name {
        $rule0 ;
        $rule1 ;
        // …
        $ruleN ;
    }
     */

    // This matches if and only if the input is also empty (i.e. four!(), four![] or four!{}).
    macro_rules! four {
        () => {4};
    }

    /**
    macro_rules! one_expression {
    ($e:expr) => {...};
    }
    **/
    macro_rules! time_five {
        ($e:expr) => {5 * $e};
    }

    macro_rules! multiply_add {
        ($a:expr, $b:expr, $c:expr) => {$a*($b + $c)};
    }

    // https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html
    macro_rules! vec_str {
        (
            // start a repetition
            $(
            // Each repeat must contain an expression
                $element:expr
            )
            // separated by commas
            ,
            // zero or more times
            *
        ) => {
            // Enclose the expansion in a block so that we can use
        // multiple statements.
            {
                let mut v = Vec::new();
                // start a repetition
                $(
                 // Each repeat will contain the following statement, with
                // $element replaced with the corresponding expression.
                  v.push(format!("{}", $element));
                )*

                v
            }
        };
    }

    macro_rules! capture_and_stringify {
        ($e:expr) => {
            stringify!($e)
        };
    }


    #[test]
    fn test() {
        let f = four![];
        let f2 = four!();
        let f3 = four! {};
        println!("{f:?}, {f2:?}, {f3:?}");

        let f4 = time_five!(3);
        println!("{f4:?}");

        let f5 = multiply_add!(2, 3, 4);
        let f6 = multiply_add!(2.0, 3.0, 4.0);
        println!("{f5:?}, {f6:?}");

        let f7 = vec_str!(10, 12, 13);
        let f8 = vec_str![10.0, 13.0, 14.0];
        println!("f7={f7:?}, f8={f8:?}");

        println!("{:?}", stringify!(dummy(2*(1+(3)))));
        println!("{:?}", capture_and_stringify!(dummy(2*(1+(3)))));
    }
}