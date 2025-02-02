# Learn about `Rust` with Deck Project

## Deck Object
### This is functionality
- new() <- create a new `deck` object that contains a list of playing cards
- shuffle() <- shuffles the order of cards in this deck
- deal() <- removes some playing cards from this deck and returns them in a new list

## Example
### Define with `Deck`
```bash
struct Deck(1) {
    cards(2): Vec<String>(3)
}
```
1. Name of the struct, always capitalized
2. List of fields (data) that this struct will wrap up
3. Vector that will contain strings.
    - Vectors are like arrays that can `grow/shrink` in size
    - Rust also has `arrays`. They have `fixed lengths`
### Make variable refer to struct
```bash
let deck(1): Deck(2) = Deck { cards: vec![](3) }(4);
```
1. Declares a new `binding` (variable)
2. Type annotation. Describes the type of value `deck` refers to
3. Creates an empty vector. Again, the `!` indicates a macro  | can using like this `Vector::new()` instead
4. Struct literal. Creates an instance of a struct
### Using variable
```bash
println!("Heres your deck: {}", deck);
```
### Fix display
```bash
#[derive(Debug)]
...
fn main() {
    ...
    println!("Heres your deck: {:?}", deck);
}
```
`[derive(2)(Debug(3))](1)`
1. `[derive(Debug)]` Whole statement defines `attributes` for the Deck struct. Gives the rest compiler some extra instructions
2. `derive` Called the `derive attribute`. Specifies which `traits` to automatically implement for this struct
3. `Debug` Called the `Debug` trait. Traits a set of functions