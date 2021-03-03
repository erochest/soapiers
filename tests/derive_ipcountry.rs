use spectral::prelude::*;
use soapier::{self, wsdl};

#[wsdl("./fixtures/iptocountry.wsdl")]

#[test]
fn test_creates_find_country_as_string() {
    
}