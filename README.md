# miniature-eureka
This tool finds two set strings in a file and replaces every line between them with the contents of another file.

## Usage
```bash
miniature-eureka path/to/source path/to/destination --start "SECTION-REPLACE-START" --end "SECTION-REPLACE-END"
```

Or if you want to search for those _specific_ strings
```bash
miniature-eureka path/to/source path/to/destination
```

If you want to be extra explicit / don't want to provide the arguments in order
```bash
miniature-eureka --src path/to/source --dst path/to/destination --start "SECTION-REPLACE-START" --end "SECTION-REPLACE-END"
```

The arguments are as follows:

- Source File
    - `--src`
    - `--source`
    - The first free standing argument found
- Destination File
    - `--dst`
    - `--dest`
    - `--destination`
    - The second free standing argument found
- Start replacing marker
    - `--start`
    - `--start-marker`
- End replacing marker
    - `--end`
    - `--end-marker`

The tool also accepts `-h` and `--help`

## `To Do`s
- Perhaps allowing to specify the src / dest file as stdin would be nice
- Optimization, currently everything is read into a string, I'm sure there is some way to instead replace in place