# EDGlass
EDGlass is an attempt to build a better Galaxy Map and to store galactic information much more efficiently. 

## Performance
EDGlass will be able to dedicate the majority of its codebase to the operation of the GalaxyMap, written in Rust Bevy
to allow for minimal unnecessary overhead without giving myself too much work.

## Tools
EDGlass plans to provide analytical tools to assist with theory crafting and discoveries such as
- Triangulation
- Bisecting
- Path tracing
- Orthographic views
- Graph generation

## Galactic Data Storage
The Galaxy is extremely vast, and there are several pre-existing services that wrangle this data. Unfortunately,
the data is provided to the public in JSON. JSON is a very accessible format and is one of the best choices for 
providing easily accessible data, especially when handling the smaller samples, but falls short when dealing with
galactic scale data (Gigabytes of information). This makes it unrealistic to locally store and handle galactic data
without creating a service for it, due to storage limitations and download speed issues. 
Thus comes in the new .gxy (dot Galaxy) format. While EDGlass will be able to import and export JSON, it will
primarily operate on this new format.

### .gxy
**Disclaimer:** EDGlass will provide a library and documentation on .gxy to create frictionless usage of the format.
However, .gxy files can be converted to JSON with the library or app for maximum compatibility with other services.

The main two perks to the galaxy format is the usage of radial coordinates and having a bespoke binary format.
While the radial coordinates lose precision the further out the star is, this doesn't come into effect without a much
larger galaxy.