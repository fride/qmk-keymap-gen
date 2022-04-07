// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/#declarativemacrosinrust
macro_rules!  combo{
    ( name = $a:expr, key_position = [ $( $idx:expr )* ] , behavior = $b:expr )  =>{
        {
            let mut key_indexes : Vec<KeyIndex> = vec![];
            $( key_indexes.push( $idx );)*
            Combo {
                name: format!("{}", $a),
                key_indexes: key_indexes,
                behavior: $b
            }
        }
   }
}

// https://stackoverflow.com/questions/62837767/rust-how-to-use-derive-macro-to-parse-the-meta-information-of-a-struct
// https://stackoverflow.com/questions/32710187/how-do-i-get-an-enum-as-a-string
macro_rules! enum_str {
    (
        $(#[$meta:meta])*
        $v:vis enum $name:ident {
        $(
            $(#[$variant_meta:meta])*
             $variant:ident
        ),*,
    }) => {
        $(#[$meta])*
        $v enum $name {
            $(
                $(#[$variant_meta])*
                $variant
            ),*
        }

        impl $name {
            fn name(&self) -> String {
                match self {
                    $($name::$variant => stringify!($variant).to_case(Case::UpperSnake)),*
                }
            }
        }
    };
}

// key press
macro_rules! kp {

    ( A ) => {
        KeyBehaviour::Press { key: KcA, mods: vec![] }
    };

    ( $k:ident )  => {

        KeyBehaviour::Press { key: $k, mods: vec![] }
    };

    ( $k:ident,  mods( $( $m:ident ), *)  )  =>{
        {
            let mut mods : Vec<Modifier> = vec![];
            $( mods.push( Modifier::$m );)*
            KeyBehaviour::Press { key: $k, mods }
        }
    }

}
