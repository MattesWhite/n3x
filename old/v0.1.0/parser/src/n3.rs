#[derive(Parser)]
#[grammar = "n3.pest"]
pub struct Parser;

#[cfg(test)]
mod test {
    use super::*;
    use pest::Parser as _;
    use test_case::test_case;

    const N3DOC: &str = r#"@prefix math: <http://www.w3.org/2000/10/swap/math#> .
    @prefix ex: <http://example.org/> .
    
    [] ex:x "2" ; ex:y "3" ; ex:z "4" .
    
    {
      ?point ex:x ?x ; ex:y ?y ; ex:z ?z .
      (?x "2") math:exponentiation ?ex .
      (?y "2") math:exponentiation ?ey .
      (?z "2") math:exponentiation ?ez .
      (?ex ?ey ?ez) math:sum ?sum .
      ?sum math:sqrt ?sqrt .
    } => {
      ex:result ex:value ?sqrt .
    } ."#;

    const FULL_DOC: &str = r#"@prefix rdf:  <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#>.
    @prefix sosa: <http://www.w3.org/ns/sosa/> .
    @prefix ssn: <http://www.w3.org/ns/ssn/> .
    @prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
    @prefix qudt-1-1: <http://qudt.org/1.1/schema/qudt#> .
    @prefix qudt-unit-1-1: <http://qudt.org/1.1/vocab/unit#> .
    @base <http://example.org/data/> .
    
    # rangefinder #30 is a laser range finder sensor that was used 
    # to observe the height of tree #124 and #125.
    
    <rangefinder/30>        rdf:type           sosa:Sensor ;
      rdfs:label "rangefinder #30"@en ;
      rdfs:comment "rangefinder #30 is a laser range finder sensor."@en .
    
    # rangefinder #30 made observation #1087 of the height of tree #124.
    
    <observation/1087>  rdf:type               sosa:Observation ;
      rdfs:label "observation #1087"@en ;
      sosa:hasFeatureOfInterest  <tree/124> ;
      sosa:observedProperty  <tree/124#height> ;
      sosa:madeBySensor <rangefinder/30> ;
      sosa:hasResult [ 
        qudt-1-1:unit qudt-unit-1-1:Meter ; 
        qudt-1-1:numericalValue "15.3"^^xsd:double ] .
    
    <tree/124>  rdf:type         sosa:FeatureOfInterest ;
      rdfs:label "tree #124"@en .
    
    <tree/124#height>  rdf:type    sosa:ObservableProperty , ssn:Property ;
      rdfs:label "the height of tree #124"@en ."#;

    const PREFIX_ID: &str = r#"@prefix rdf:  <http://www.w3.org/1999/02/22-rdf-syntax-ns#> ."#;
    const BASE: &str = r#"@base <http://example.org/data/> ."#;
    const ANON_STATEMENT: &str = r#"[] ex:x "2" ; ex:y "3" ; ex:z "4" ."#;
    const TRIPLES: &str = r#"<rangefinder/30>        rdf:type           sosa:Sensor ;
        rdfs:label "rangefinder #30"@en ;
        rdfs:comment "rangefinder #30 is a laser range finder sensor."@en ."#;
        
    #[test_case(N3DOC, Rule::document => true ;    "document n3")]
    #[test_case(FULL_DOC, Rule::document => true ; "document turtle")]
    #[test_case(PREFIX_ID, Rule::statement => true ; "statement")]
    #[test_case(PREFIX_ID, Rule::directive => true ; "directive")]
    #[test_case(PREFIX_ID, Rule::prefixID => true ; "prefix_id")]
    #[test_case(BASE, Rule::base => true ; "base")]
    #[test_case(TRIPLES, Rule::simple_statement => true ;        "simple_statement triples")]
    #[test_case(ANON_STATEMENT, Rule::simple_statement => true ; "simple_statement anon")]
    #[test_case("\"quote\"", Rule::String => true;          "String quote")]
    #[test_case("'quote'", Rule::String => true ;           "String single")]
    #[test_case("\"\"\"quote\"\"\"", Rule::String => true ; "String long quote")]
    #[test_case("'''quote'''", Rule::String => true ;       "String long single quote")]
    #[test_case("<>", Rule::IRIREF => true ;             "IRIREF empty sting")]
    #[test_case("<http://www.w3.org/1999/02/>", Rule::IRIREF => true ; "IRIREF IRI")]
    #[test_case("<http://www.w3.org/1999/02/22-rdf-syntax-ns#>", Rule::IRIREF => true ; "IRIREF IRI ending with '#'")]
    #[test_case("<../ns/vocab#>", Rule::IRIREF => true ; "IRIREF relative IRI")]
    #[test_case("<\\u0ace>", Rule::IRIREF => true ;      "IRIREF numeric escape small")]
    #[test_case("<\\UFeDc0123>", Rule::IRIREF => true ;  "IRIREF numeric escape big")]
    #[test_case("<\0>", Rule::IRIREF => false ;          "IRIREF null character")]
    #[test_case("<  >", Rule::IRIREF => false ;          "IRIREF space")]
    #[test_case("<\">", Rule::IRIREF => false ;          "IRIREF quote")]
    #[test_case("<{>", Rule::IRIREF => false ;           "IRIREF open curly")]
    #[test_case("<}>", Rule::IRIREF => false ;           "IRIREF close curly")]
    #[test_case("<|>", Rule::IRIREF => false ;           "IRIREF bar")]
    #[test_case("<^>", Rule::IRIREF => false ;           "IRIREF caret")]
    #[test_case("<`>", Rule::IRIREF => false ;           "IRIREF back tick")]
    #[test_case("<\\>", Rule::IRIREF => false ;          "IRIREF backslash")]
    #[test_case("<\\u000>", Rule::IRIREF => false ;      "IRIREF numeric escape small less digits")]
    #[test_case("<\\uzzzz>", Rule::IRIREF => false ;     "IRIREF numeric escape small wrong digits")]
    #[test_case("<\\U000000>", Rule::IRIREF => false ;   "IRIREF numeric escape big less digits")]
    #[test_case("<\\Uzzzzzzzz>", Rule::IRIREF => false ; "IRIREF numeric escape big wrong digits")]
    #[test_case(":", Rule::PNAME_NS => true ;                   "PNAME_NS empty prefix")]
    #[test_case("rBäôí:", Rule::PNAME_NS => true ;              "PNAME_NS alpha")]
    #[test_case("rBäôí.:", Rule::PNAME_NS => false ;            "PNAME_NS prefix ends with dot")]
    #[test_case("rBäôí :", Rule::PNAME_NS => false ;            "PNAME_NS alpha with space")]
    #[test_case("rBäôí", Rule::PNAME_NS => false ;              "PNAME_NS alpha without colon")]
    #[test_case("a_-:", Rule::PNAME_NS => true ;                "PNAME_NS contain allowed special")]
    #[test_case("a0123456789:", Rule::PNAME_NS => true ;        "PNAME_NS contain numeric")]
    #[test_case("a0123456789", Rule::PNAME_NS => false ;        "PNAME_NS contain numeric without colon")]
    #[test_case("a01.23.456.789:", Rule::PNAME_NS => true ;     "PNAME_NS with dots")]
    #[test_case("a_01.2-3.45ö6.78î9:", Rule::PNAME_NS => true ; "PNAME_NS mixed")]
    #[test_case("", Rule::PNAME_NS => false ;                   "PNAME_NS empty")]
    #[test_case("0:", Rule::PNAME_NS => false ;                 "PNAME_NS star numeric")]
    #[test_case("_:", Rule::PNAME_NS => false ;                 "PNAME_NS star allowed special")]
    #[test_case("!?:\\,.<>#", Rule::PNAME_NS => false ;         "PNAME_NS unallowed special")]
    #[test_case(" ", Rule::PNAME_NS => false ;                  "PNAME_NS space")]
    #[test_case("_:example", Rule::BLANK_NODE_LABEL => true ; "BLANK_NODE_LABEL start alpha")]
    #[test_case("_:0", Rule::BLANK_NODE_LABEL => true ;       "BLANK_NODE_LABEL start num")]
    #[test_case("_:_", Rule::BLANK_NODE_LABEL => true ;       "BLANK_NODE_LABEL start under")]
    #[test_case("@en", Rule::LANGTAG => true ;          "LANGTAG simple")]
    #[test_case("@en-uk", Rule::LANGTAG => true ;       "LANGTAG expanded")]
    #[test_case("@en-uk-man", Rule::LANGTAG => true ;   "LANGTAG further")]
    #[test_case("en-uk-man", Rule::LANGTAG => false ;   "LANGTAG missing at")]
    #[test_case("@1en-uk-man", Rule::LANGTAG => false ; "LANGTAG number in first")]
    #[test_case("@en-2uk2-man", Rule::LANGTAG => true ; "LANGTAG number in second")]
    #[test_case("@en-", Rule::LANGTAG => true ;         "LANGTAG no extension")]
    #[test_case(" ", Rule::LANGTAG => false ;           "LANGTAG space")]
    #[test_case("123", Rule::INTEGER        => true ;  "INTEGER integer")]
    #[test_case("-123", Rule::INTEGER       => true ;  "INTEGER ninteger")]
    #[test_case("123.45", Rule::INTEGER     => true ;  "INTEGER decimal")]
    #[test_case("-123.45", Rule::INTEGER    => true ;  "INTEGER ndecimal")]
    #[test_case(".45", Rule::INTEGER        => false ; "INTEGER decimal dot")]
    #[test_case("-.45", Rule::INTEGER       => false ; "INTEGER ndecimal dot")]
    #[test_case("1.2345e2", Rule::INTEGER   => true ;  "INTEGER double")]
    #[test_case("-12345E-2", Rule::INTEGER  => true ;  "INTEGER ndouble")]
    #[test_case("-.12345E-2", Rule::INTEGER => false ; "INTEGER ndouble dot")]
    #[test_case(" ", Rule::INTEGER          => false ; "INTEGER space")]
    #[test_case("123", Rule::DECIMAL        => false ; "DECIMAL integer")]
    #[test_case("-123", Rule::DECIMAL       => false ; "DECIMAL ninteger")]
    #[test_case("123.45", Rule::DECIMAL     => true ;  "DECIMAL decimal")]
    #[test_case("-123.45", Rule::DECIMAL    => true ;  "DECIMAL ndecimal")]
    #[test_case(".45", Rule::DECIMAL        => true ;  "DECIMAL decimal dot")]
    #[test_case("-.45", Rule::DECIMAL       => true ;  "DECIMAL ndecimal dot")]
    #[test_case("1.2345e2", Rule::DECIMAL   => true ;  "DECIMAL double")]
    #[test_case("-12345E-2", Rule::DECIMAL  => false ; "DECIMAL ndouble")]
    #[test_case("-.12345E-2", Rule::DECIMAL => true ;  "DECIMAL ndouble dot")]
    #[test_case(" ", Rule::DECIMAL          => false ; "DECIMAL space")]
    #[test_case("123", Rule::DOUBLE        => false ; "DOUBLE integer")]
    #[test_case("-123", Rule::DOUBLE       => false ; "DOUBLE ninteger")]
    #[test_case("123.45", Rule::DOUBLE     => false ; "DOUBLE decimal")]
    #[test_case("-123.45", Rule::DOUBLE    => false ; "DOUBLE ndecimal")]
    #[test_case(".45", Rule::DOUBLE        => false ; "DOUBLE decimal dot")]
    #[test_case("-.45", Rule::DOUBLE       => false ; "DOUBLE ndecimal dot")]
    #[test_case("1.2345e2", Rule::DOUBLE   => true ;  "DOUBLE double")]
    #[test_case("-12345E-2", Rule::DOUBLE  => true ;  "DOUBLE ndouble")]
    #[test_case("-.12345E-2", Rule::DOUBLE => true ;  "DOUBLE ndouble dot")]
    #[test_case(" ", Rule::DOUBLE          => false ; "DOUBLE space")]
    #[test_case("[]", Rule::ANON => true ;            "ANON no space")]
    #[test_case("[ \t\n]", Rule::ANON => true ;       "ANON valid space")]
    #[test_case("[ \thello\n]", Rule::ANON => false ; "ANON not empty")]
    #[test_case(" \t\n", Rule::ANON => false ;        "ANON no brackets")]
    #[test_case("rBäôí", Rule::PN_CHARS_BASE => true ;          "PN_CHARS_BASE alpha")]
    #[test_case("", Rule::PN_CHARS_BASE => false ;              "PN_CHARS_BASE empty")]
    #[test_case("0123456789", Rule::PN_CHARS_BASE => false ;    "PN_CHARS_BASE numeric")]
    #[test_case("_!?-:\\,.-<>#", Rule::PN_CHARS_BASE => false ; "PN_CHARS_BASE special")]
    #[test_case(" ", Rule::PN_CHARS_BASE => false ;             "PN_CHARS_BASE space")]
    #[test_case("", Rule::VARNAME => false ;          "VARNAME empty")]
    #[test_case("hans", Rule::VARNAME => true ;       "VARNAME alpha")]
    #[test_case("_", Rule::VARNAME => true ;          "VARNAME underscore")]
    #[test_case("1", Rule::VARNAME => true ;          "VARNAME number")]
    #[test_case("hans_the_1", Rule::VARNAME => true ; "VARNAME mixed")]
    #[test_case("rBäôí", Rule::PN_CHARS_U => true ;         "PN_CHARS_U alpha")]
    #[test_case("_", Rule::PN_CHARS_U => true ;             "PN_CHARS_U allowed special")]
    #[test_case("", Rule::PN_CHARS_U => false ;             "PN_CHARS_U empty")]
    #[test_case("0123456789", Rule::PN_CHARS_U => false ;   "PN_CHARS_U numeric")]
    #[test_case("!?-:\\,.-<>#", Rule::PN_CHARS_U => false ; "PN_CHARS_U unallowed special")]
    #[test_case(" ", Rule::PN_CHARS_U => false ;            "PN_CHARS_U space")]
    #[test_case("rBäôí", Rule::PN_CHARS => true ;       "PN_CHARS alpha")]
    #[test_case("_-", Rule::PN_CHARS => true ;          "PN_CHARS allowed special")]
    #[test_case("", Rule::PN_CHARS => false ;           "PN_CHARS empty")]
    #[test_case("0123456789", Rule::PN_CHARS => true ;  "PN_CHARS numeric")]
    #[test_case("!?:\\,.<>#", Rule::PN_CHARS => false ; "PN_CHARS unallowed special")]
    #[test_case(" ", Rule::PN_CHARS => false ;          "PN_CHARS space")]
    #[test_case("rBäôí", Rule::PN_PREFIX => true ;              "PN_PREFIX alpha")]
    #[test_case("rBäôí.", Rule::PN_PREFIX => true ;             "PN_PREFIX end with dot")]
    #[test_case("a_-", Rule::PN_PREFIX => true ;                "PN_PREFIX contain allowed special")]
    #[test_case("a0123456789", Rule::PN_PREFIX => true ;        "PN_PREFIX contain numeric")]
    #[test_case("a01.23.456.789", Rule::PN_PREFIX => true ;     "PN_PREFIX with dots")]
    #[test_case("a_01.2-3.45ö6.78î9", Rule::PN_PREFIX => true ; "PN_PREFIX mixed")]
    #[test_case("", Rule::PN_PREFIX => false ;                  "PN_PREFIX empty")]
    #[test_case("0", Rule::PN_PREFIX => false ;                 "PN_PREFIX star numeric")]
    #[test_case("_", Rule::PN_PREFIX => false ;                 "PN_PREFIX star allowed special")]
    #[test_case("!?:\\,.<>#", Rule::PN_PREFIX => false ;        "PN_PREFIX unallowed special")]
    #[test_case(" ", Rule::PN_PREFIX => false ;                 "PN_PREFIX space")]
    #[test_case("%ab", Rule::PLX => true ;  "PLX hex valid")]
    #[test_case("%yz", Rule::PLX => false ; "PLX hex invalid")]
    #[test_case("\\.", Rule::PLX => true ;  "PLX escape")]
    #[test_case(".", Rule::PLX => false ;   "PLX unescape")]
    #[test_case(" ", Rule::PLX => false ;   "PLX space")]
    #[test_case("%ab", Rule::PERCENT => true ;  "PERCENT hex valid")]
    #[test_case("%yz", Rule::PERCENT => false ; "PERCENT hex invalid")]
    #[test_case(" ", Rule::PERCENT => false ;   "PERCENT space")]
    #[test_case("\\.", Rule::PN_LOCAL_ESC => true ; "PN_LOCAL_ESC escape")]
    #[test_case(".", Rule::PN_LOCAL_ESC => false ;  "PN_LOCAL_ESC unescape")]
    #[test_case(" ", Rule::PN_LOCAL_ESC => false ;  "PN_LOCAL_ESCspace")]
    fn parse_rule(text: &str, rule: Rule) -> bool {
        Parser::parse(rule, text)
            .map(|pairs| {
                println!("XXX Parsed:\n{:#?}\nXXX", pairs);
            })
            .map_err(|e| println!("Error occured:\n{}\n", e))
            .is_ok()
    }
}
