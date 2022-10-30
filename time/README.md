This program was created with the help of avr-hal template, which automatically
creates an easy to modify template for avr projects. Guide for using the template for outside projects
programs: https://github.com/Rahix/avr-hal-template.git

## Running the program
If you don't have them already, install  [`ravedude`]:

```bash
cargo install ravedude
```
Once ravedude has been installed, avr board can be flashed. This project requires an arduino atmega2560.
The template has the required flags for flashing the board, so running the program flashes the board
automatically.

```bash
cargo run
```

[`ravedude`]: https://github.com/Rahix/avr-hal/tree/next/ravedude

## Program description.
This project creates the millis function for arduino, reads the value of it while reading bytes of data (3 characters + enter is the limit) and says the ascii value and time.
While data is being read, the onboard led is on. There is 1 second delay per character, so it can be more easily noticed. Once the program finishes reading input, it goes to a powersafe mode.
