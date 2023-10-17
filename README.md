# Mensa UPB CLI

Simple CLI for the [Mensa UPB](https://www.studierendenwerk-pb.de/gastronomie/speiseplaene) website.

## Installation

```bash
cargo install --git https://github.com/moritz-hoelting/mensa-upb-cli
```

## Usage

### Show the menu for today
```bash
mensa-upb-cli
```

### Show the help screen
```bash
mensa-upb-cli --help
```

### Show the menu of the third next day
```bash
mensa-upb-cli -d 3
```

### Show the menu of a different mensa
```bash
mensa-upb-cli -m grill-cafe
```

### Show the only the prices for students
```bash
mensa-upb-cli -p student
```