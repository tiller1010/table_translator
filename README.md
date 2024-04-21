# Translator App for Fluent CLM
Rust app to translate HTML content from a SilverStripe Content Layout Module into Fluent tables.
- Generates an english_original.csv file from a MySQL query result
- Parses HTML content
- Uses Yandex translation crate to translate from English to desired language
- Outputs a {DESIRED_LANG}_output.csv file
- Inserts the translated HTML into Fluent tables

## Usage
First create the US English default and the desired language locales in SilverStripe and set to the home country variant (e.g. Spain Spanish).

Then generate the English CSV source file with `cargo run generate_source`

Next, translate the CSV file with `cargo run translate <locale>` where `<locale>` is the locale you want to translate to. This defaults to Spanish. Here is a list of the available locales:
```rs
"es_ES" => Language::Spanish,
"fr_FR" => Language::French,
"de_DE" => Language::German,
"it_IT" => Language::Italian,
"pt_PT" => Language::Portuguese,
"ru_RU" => Language::Russian,
"tr_TR" => Language::Turkish,
"zh_CN" => Language::Chinese,
```

Finally, insert the translated records into the appropriate Fluent tables with `cargo run insert_translations <locale>`. This defaults to Spanish.

Shortcut to run all: `cargo run all es_ES`
