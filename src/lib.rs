#[macro_use]
extern crate ruru;
extern crate woothee;

use ruru::{Symbol, Boolean, RString, Class, Hash, Object};

use woothee::parser::Parser;

class!(FastWoothee);

methods!(
    FastWoothee,
    _itself,
    fn parse(ua_string: RString) -> Hash {
        let mut hash = Hash::new();

        let parser = Parser::new();
        match parser.parse(ua_string.unwrap_or(RString::new("")).to_str()) {
            None => hash,
            Some(result) => {
                hash.store(Symbol::new("name"), RString::new(result.name));
                hash.store(Symbol::new("category"), RString::new(result.category));
                hash.store(Symbol::new("os"), RString::new(result.os));
                hash.store(Symbol::new("os_version"), RString::new(&*result.os_version));
                hash.store(Symbol::new("browser_type"), RString::new(result.browser_type));
                hash.store(Symbol::new("version"), RString::new(&*result.version));
                hash.store(Symbol::new("vendor"), RString::new(result.vendor));
                hash
            }
        }
    }
);

methods!(
    FastWoothee,
    _itself,
    fn crawler(ua_string: RString) -> Boolean {
        Boolean::new(woothee::is_crawler(ua_string.unwrap_or(RString::new("")).to_str()))
    }
);

#[no_mangle]
pub extern fn initialize_fast_woothee() {
    Class::new("FastWoothee", None).define(|itself| {
        itself.const_set("VERSION", &RString::new("1.1.0").freeze());
        itself.def_self("parse", parse);
        itself.def_self("crawler?", crawler);
    });
}
