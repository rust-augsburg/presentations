#set raw(theme: "dark.tmTheme")
#set page(
    paper: "a6",
    margin: (x: 10mm, y: 12mm),
    flipped: true,
    background: image("background.svg"),
    footer: [
        #set text(10pt)
        #box(
            width: 84.0%,
            [
                #set align(left)
                Rust Meetup Augsburg - 31.8.2023
            ]
        )
        #box(
            width: 15.0%,
            [
                #set align(right)
                #set text(9pt, font: "JetBrains Mono")
                #counter(page).display(
                  "1/1",
                  both: true,
                )
            ]
        )
    ]
)
#set text(
    font: "IBM Plex Sans",
    fill: rgb(255, 255, 255),
    lang: "en",
    ligatures: true,
)
#set par(leading: 1em)

#show raw: text.with(
    size: 1.1em,
    font: "JetBrains Mono",
    ligatures: true,
)
#show raw.where(block: true): par.with(
    leading: 0.7em,
)

#block([
#set align(horizon)
#v(5.5em)
= #underline("Skalierbares Design prozeduraler Makros", offset: 0.5mm, stroke: 0.01mm)
#text(9pt, style: "italic", "Scalable design of procedural macros")
#set align(center)
#v(1em)
#image("rustacean-flat-happy.png", width: 14em)
])

#let chapters = (
    what: "What are proc-macros",
    how: "How do proc-macros work?",
    crates: "Common proc-macro crates",
    design: "Scalable design",
    shortcomings: "Shortcomings",
)

#let presentation_header(num) = [
    #set align(top)
    #v(1.7em)
    #text(
        size: 8pt, 
        style: "italic", 
        fill: gray,
        chapters.values().at(num)
    )
]

#pagebreak()

= Content
#v(1em)

#for chapter in chapters.values() [
    + #chapter
]

#pagebreak()

#set page(header: [#presentation_header(0)])

= Why do we need macros?
#v(1em)

#table(
    columns: (50%, auto),
    inset: 0pt,
    row-gutter: 10pt,
    stroke: none,
    align: top,
    [*Without macros*], [*With macros*],
    [
```rust
let a = {
   let mut v = Vec::new();
   v.push(1);
   v.push(2);
   v.push(3);
   v
};
```
    ], [
```rust
macro_rules! vec {
   ($($x:expr),+) => ({
       let mut v = Vec::new();
       $( v.push($x); )+
       v
   });
}

let a = vec![0, 1, 2];
let b = vec![3, 4, 5];
let c = vec![6, 7, 9];
// ...
```
    ]
)


#pagebreak()

#box(height: 100%,[
#set align(horizon)
= Who uses macros regularly?
#v(1em)
])

#pagebreak()

= Commonly used proc-macros
#v(1em)

- serde: #box[```rust
#[derive(Serialize)]
```]
- thiserror: #box[```rust
#[derive(Error)]
```]
- async_trait: #box[```rust
#[async_trait]
```]
- tokio: #box[```rust
#[tokio::main]
```]
- #box[
Relm4: ```rust
view! {
    gtk::Box {
        gtk::Label {
            set_label: "This is a label",
        }
    }
}
```]

= Some numbers
#v(1em)

`syn` is the most downloaded crate with over \
#text(weight: "bold", style: "italic", "13.000.000") downloads per month.

#pagebreak()

#set page(header: [#presentation_header(1)])

= Proc-macro architecture
#v(2em)

#table(
    columns: (auto, auto),
    inset: 0em,
    column-gutter: 2em,
    row-gutter: 2em,
    stroke: none,
    [- declarative macros],
    [
$ "syntax patterns" ==> "Rust code" $
    ],
    [- procedural macros],
    [
$ "tokens" underbrace(==>, "Rust code") "Rust code" $
    ]
)

#pagebreak()

= Function-like proc-macros
#v(1em)

```rust
#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// ...

make_answer!()
```

#pagebreak()

= Derive proc-macros
#v(1em)

```rust
#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// ...

#[derive(AnswerFn)]
struct Struct {
    // ...
}
```

#pagebreak()

= Attribute proc-macros
#v(1em)

```rust
#[proc_macro_attribute]
pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

// ...

#[return_as_is]
struct MyStruct;

#[return_as_is(further_attrs)]
enum MyEnum {}
```

#pagebreak()

= Valid syntax
#v(1em)

#table(
    columns: (auto, auto),
    inset: 0cm,
    row-gutter: 2em,
    column-gutter: 1em,
    stroke: none,
[Invalid],
[Valid],
[
```rs
#[quick_methods]
struct MyStruct {
    some_value: u8,
    method => |_| "test",    
}
```
],
[
```rs
quick_methods! {
    struct MyStruct {
        some_value: u8,
        method => |_| "test",    
    }
}
```
],
)
#pagebreak()

= Valid syntax
#v(1em)

Invalid: Brackets must match
```rs
c_code! {
    #define closing_bracket }
    {
        int i = 0;
    closing_bracket
}
```

#pagebreak()

#set page(header: [#presentation_header(2)])

= The `proc-macro` crate
#v(1em)

- Part of the Rust toolchain
- Link between rustc and proc-macro library
- Very basic
- Only works in proc-macros (no tests)
- #box[Based on `TokenTree`:

```rs
pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
}
```
]

#pagebreak()

= The `proc-macro2` crate
#v(1em)

- Wrapper on top of `proc-macro`
- Works outside of proc-macros
- Can be used in in `#[test]` or build.rs

#pagebreak()

= The `syn` crate
#v(1em)

- Convenient parsing
- Pre-defined items (struct, enum, impl, ...)
- Utilities for processing and error handling

#pagebreak()

= The `syn` crate - `Parse` trait
#v(1em)

```rust
struct ItemStruct {
    struct_token: Token![struct],
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}
```
#pagebreak()


= The `syn` crate - `Parse` trait
#v(1em)

```rust
impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(ItemStruct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse_named, Token![,])?,
        })
    }
}
```

#pagebreak()


= The `quote` crate
#v(1em)

- Convert tokens and `syn` data structures into tokens
- #box[Produces `TokenStream`s for the code generation of the macro
```rs
let struct_name = "MyStruct";
let field_name = format_ident!("_{}", ident);
quote! {
    struct #struct_name {
        #field_name: u8;
    } 
}
```
]

#set page(header: [#presentation_header(3)])

#pagebreak()

= Anti-patterns
#v(1em)

- Unfamiliar syntax
  - Higher learning curve
  - More custom parsing
- Highly conditional parsing
  - #box[ Avoid conditional syntax
```rust
pub(crate) ? // struct | enum | type ...
```
]
  - #box[Don't use context-dependent parsing
```rust
#[my_macro()]
enum MyEnum {} // Works
#[my_macro("only-structs-please")]
enum MyEnum2 {} // Error
```
  ]

#pagebreak()

= Good design patterns
#v(1em)

+ 3-step processing: Parsing -> Logic -> Code-generation
+ Multiple streams
+ Spanned tokens/errors
+ Error recovery/Fallback
+ Visitors

#pagebreak()

= Good design patterns - 3-step parsing
#v(1em)

+ `main.rs`
  + Entry point
  + Type definitions for parsing
+ `parse.rs`
  + `Parse` implementations
+ `gen.rs`
  + Logic
  + Code generation

#pagebreak()

= Good design patterns - Spanned errors
#v(1em)

```rust
#[crate::track]
struct Test {
    // This is needed -> #[tracker::no_eq]
    c: Empty,
}
```

#image("error.png")

#pagebreak()

= Good design patterns - Error recovery
#v(1em)

```rust
match MyStruct::parse(input) {
    Ok(my_struct) => my_struct.generate_code(),
    Err(err) => {
        quote! {
            impl SomeTrait for MyStruct {
                fn some_method() {
                    todo!()
                }
            }
            #err
        }
    }
}
```

#pagebreak()

#set page(header: [#presentation_header(4)])

= Shortcomings
#v(1em)

- Difficult to get right
- Hard to read and review
- Black box for the compiler
  - Bad language server integration
- Sandboxing
- Bad hygiene
  - Clean imports only through `use ::crate_name`;
  - Does not work well with re-exports
- Parsing logic can't be used for writing formatters
