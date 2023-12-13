enum TimeOfDay {
    // enum
    Dawn,
    Day,
    Sunset,
    Night,
}

fn change_fear(input: TimeOfDay) -> impl FnMut(f64) -> f64 {
    // la función recibe TimeOfDay. Devuelve un closure.
    // Se usa impl FnMut(64) -> f64 para indicar la función que
    // cambiará el valor
    use TimeOfDay::*; // Para que sea más corto de escribir Dawn, Day, Sunset, Night
                      // En lugar de TimeOfDay::Dawn, TimeOfDay::Day, etc.
    match input {
        Dawn => |x| {
            // x representa a la variable character_fear que se pasará después
            println!(
                "The morning sun has vanquished the horrible night. You no longer feel afraid."
            );
            println!("Your fear is now {}", x * 0.5);
            x * 0.5
        },
        Day => |x| {
            println!("What a nice day. Maybe put your feet up and rest a bit.");
            println!("Your fear is now {}", x * 0.2);
            x * 0.2
        },
        Sunset => |x| {
            println!("The sun is almost down! This is no good.");
            println!("Your fear is now {}", x * 1.4);
            x * 1.4
        },
        Night => |x| {
            println!("What a horrible night to have a curse.");
            println!("Your fear is now {}", x * 5.0);
            x * 5.0
        },
    }
}

fn main() {
    use TimeOfDay::*;
    let mut character_fear = 10.0; // Comienza Simon a 10

    let mut daytime = change_fear(Day); // Crea cuatro cierres que cambian el nivel de miedo de Simon.
    let mut sunset = change_fear(Sunset);
    let mut night = change_fear(Night);
    let mut morning = change_fear(Dawn);

    character_fear = daytime(character_fear); // Llama a los cierres. Cambian el miedo y escriben un mensaje.
    character_fear = sunset(character_fear);
    character_fear = night(character_fear);
    character_fear = morning(character_fear);
}
