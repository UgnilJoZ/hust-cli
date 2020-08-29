#  hust CLI
Just a CLI for the hue bridge written in Rust.
## Usage
First, explore the network.
```
hust discover
```
The command will output and save the found bridges.

Then, list the lights of the found bridge:
```
hust light list
```
If there are multiple bridges in the network, you may specify a bridge by its UDN:
```
hust light -b "uuid:abcdef12-da50-11e1-9b23-ecb5fa004b9e" list
```
The output will contain each light's properties.

The light identifiers on the left side enable you now to switch lights on and off.
```
hust light switch -l 1 on
```
The `-b` switch for specifying the bridge will also work for the `light` subcommand.
```
hust light -b "uuid:abcdef12-da50-11e1-9b23-ecb5fa004b9e" switch -l 2 off
```