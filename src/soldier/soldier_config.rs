use serde::Deserialize;
/**
 Represents the configuration archive structure for the client

# Example of 'config.ron'
```
    SoldierConfig(
    name: "S. Buck",
    addr: "armly.henrybarreto.dev:14114",
    interval: 5
    )
```
*/
#[derive(Deserialize, Clone, Debug)]
pub struct SoldierConfig {
    pub name: String,
    pub addr: String,
    pub interval: u64,
}
