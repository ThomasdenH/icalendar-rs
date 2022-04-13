use super::*;

/// VVENUE  [(ical-venue)](https://tools.ietf.org/html/draft-norris-ical-venue-01)
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Venue {
    pub(super) inner: InnerComponent,
}
impl Venue {
    /// Creates a new Venue.
    pub fn new() -> Self {
        Default::default()
    }

    /// End of builder pattern.
    /// copies over everything
    pub fn done(&mut self) -> Self {
        Venue {
            inner: self.inner.done(),
        }
    }

    /// Set the STREET-ADDRESS `Property`
    ///
    /// This specifies the street address of a location. If the location requires a multiple-line
    /// address, they may be separated by an encoded newline "\n".
    pub fn street_address(&mut self, address: &str) -> &mut Self {
        self.add_property("STREET-ADDRESS", address)
    }

    /// Gets the value of the STREET-ADDRESS `Property`.
    pub fn get_street_address(&self) -> Option<&str> {
        self.property_value("STREET-ADDRESS")
    }

    /// Set the EXTENDED-ADDRESS `Property`
    ///
    /// This property provides the opportunity to include extended address information for a
    /// location. This property may be used to give additional information about an address that is
    /// not usually considered part of the street address. If the location requires a multiple-line
    /// address, they may be separated by an encoded newline "\n".
    pub fn extended_address(&mut self, address: &str) -> &mut Self {
        self.add_property("EXTENDED-ADDRESS", address)
    }

    /// Gets the value of the EXTENDED-ADDRESS `Property`.
    pub fn get_extended_address(&self) -> Option<&str> {
        self.property_value("EXTENDED-ADDRESS")
    }

    /// Set the LOCALITY `Property`
    ///
    /// This specifies the city or locality of a venue.
    pub fn locality(&mut self, locality: &str) -> &mut Self {
        self.add_property("LOCALITY", locality)
    }

    /// Gets the value of the LOCALITY `Property`.
    pub fn get_locality(&self) -> Option<&str> {
        self.property_value("LOCALITY")
    }

    /// Set the REGION `Property`
    ///
    /// This specifies the region (state, province, canton, etc.) of a location.
    pub fn region(&mut self, region: &str) -> &mut Self {
        self.add_property("REGION", region)
    }

    /// Gets the value of the REGION `Property`.
    pub fn get_region(&self) -> Option<&str> {
        self.property_value("REGION")
    }

    /// Set the COUNTRY `Property`
    ///
    /// This specifies the country of a location.
    pub fn country(&mut self, country: &str) -> &mut Self {
        self.add_property("COUNTRY", country)
    }

    /// Gets the value of the COUNTRY `Property`.
    pub fn get_country(&self) -> Option<&str> {
        self.property_value("COUNTRY")
    }

    /// Set the POSTAL-CODE `Property`
    ///
    /// This specifies the postal code of a location.
    pub fn postal_code(&mut self, postal_code: &str) -> &mut Self {
        self.add_property("POSTAL-CODE", postal_code)
    }

    /// Gets the value of the POSTAL-CODE `Property`.
    pub fn get_postal_code(&self) -> Option<&str> {
        self.property_value("POSTAL-CODE")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_properties_unset() {
        let venue = Venue::new();
        assert_eq!(venue.get_street_address(), None);
        assert_eq!(venue.get_extended_address(), None);
        assert_eq!(venue.get_locality(), None);
        assert_eq!(venue.get_region(), None);
        assert_eq!(venue.get_country(), None);
        assert_eq!(venue.get_postal_code(), None);
    }

    #[test]
    fn get_properties_set() {
        let venue = Venue::new()
            .street_address("street address")
            .extended_address("extended address")
            .locality("locality")
            .region("region")
            .country("country")
            .postal_code("postal code")
            .done();
        assert_eq!(venue.get_street_address(), Some("street address"));
        assert_eq!(venue.get_extended_address(), Some("extended address"));
        assert_eq!(venue.get_locality(), Some("locality"));
        assert_eq!(venue.get_region(), Some("region"));
        assert_eq!(venue.get_country(), Some("country"));
        assert_eq!(venue.get_postal_code(), Some("postal code"));
    }
}
