//! Standard namespaces.
//! 
//! # Example
//! ```
//! use sophia::graph::*;
//! use sophia::graph::inmem::FastGraph;
//! use sophia::term::StaticTerm;
//! 
//! let mut g = FastGraph::new();
//! let foo = StaticTerm::new_iri("http://example.org/foo").unwrap();
//!
//! use sophia::ns::{rdf, rdfs, xsd};
//! 
//! g.insert(&foo, &rdf::type_, &rdf::Property);
//! g.insert(&foo, &rdfs::range, &xsd::string);
//! ```

#[macro_export]
macro_rules! namespace {
    ($prefix:expr, $($suffix:ident),*) => {
        pub static PREFIX:&'static str = $prefix;
        $(
            ns_term!($prefix, $suffix);
        )*
    };
}

macro_rules! ns_term {
    ($prefix:expr, $ident:ident) => {
        ns_term!($prefix, $ident, stringify!($ident));
    };
    ($prefix:expr, $ident:ident, $suffix:expr) => {
        #[allow(non_upper_case_globals)]
        pub static $ident:term::StaticTerm =
            term::Term::Iri(
                term::IriData{
                    ns: $prefix,
                    suffix: Some($suffix),
                    absolute: true,
            });
    }
}

//pub static $ident:term::Term<'static> = term::Term::Iri(term::IriData{ns:$prefix, suffix:$suffix});

/// The standard `rdf:` namespace.
/// 
/// NB: since `type` is a reserved keyword in Rust,
/// the term `rdf:type` spells `rdf::type_` (with a trailing underscore).
/// 
#[allow(non_upper_case_globals)]
pub mod rdf {
    use crate::term;

    namespace!("http://www.w3.org/1999/02/22-rdf-syntax-ns#",
        // classes
        Alt, Bad, List, PlainLiteral, Property, Seq, Statement,
        // datatypes
        HTML, langString, XMLLiteral,
        // properties
        first, object, predicate, rest, subject, value,
        // individuals
        nil
    );
    ns_term!("http://www.w3.org/1999/02/22-rdf-syntax-ns#", type_, "type");
}

/// The standard `xsd:` namespace.
#[allow(non_upper_case_globals)]
pub mod xsd {
    use crate::term;

    namespace!("http://www.w3.org/2001/XMLSchema#",
    anyType,
    anySimpleType,

    duration,
    dateTime,
    time,
    date,
    gYearMonth,
    gYear,
    gMonthDay,
    gDay,
    gMonth,
    boolean,
    base64Binary,
    hexBinary,
    float,
    double,
    anyURI,
    QName,
    NOTATION,
    string,
        normalizedString,
            token,
                language,
                Name,
                    NCName,
                        ID,
                        IDREF,
                            IDREFS,
                        ENTITY,
                            ENTITIES,
                NMTOKEN,
                    NMTOKENS,
    decimal,
        integer,
            nonPositiveInteger,
                negativeInteger,
            long,
                int,
                    short,
                        byte,
            nonNegativeInteger,
                unsignedLong,
                    unsignedInt,
                        unsignedShort,
                            unsignedByte,
                positiveInteger
    );
}

/// The standard `rdfs:` namespace.
#[allow(non_upper_case_globals)]
pub mod rdfs {
    use crate::term;

    namespace!("http://www.w3.org/2000/01/rdf-schema#",
        Class, Container, ContainerMembershipProperty, Datatype, Literal, Resource,
        domain, range, subClassOf, subPropertyOf,
        comment, isDefinedBy, label, member, seeAlso
    );
}



#[cfg(test)]
mod test {
    // Nothing really worth testing here
}