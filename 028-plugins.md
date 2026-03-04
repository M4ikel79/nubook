# Plugins

Nu can be extended using plugins. Plugins behave much like Nu's built-in commands, with the added benefit that they can be added separately from Nu itself.

**Important:** Plugins communicate with Nushell using the `nu-plugin` protocol. This protocol is versioned, and plugins must use the same `nu-plugin` version provided by Nushell. When updating Nushell, please make sure to also update any plugins that you have registered.

## Overview

To use a plugin, it needs to be installed, added, and imported. There are two types of plugins:

- **Core plugins**: Officially maintained, usually installed with Nushell in the same directory as the Nushell executable
- **Third-party plugins**: Available from many sources (crates.io, Git repositories, awesome-nu)

The `$NU_PLUGIN_DIRS` constant or `$env.NU_PLUGIN_DIRS` environment variable can be used to set the search-path for plugins.

## Core Plugin Quickstart (Polars)

1. Most package managers automatically install core plugins with Nushell
2. Set the plugin search path:
   ```nu
   const NU_PLUGIN_DIRS = [
     ($nu.current-exe | path dirname)
     ...$NU_PLUGIN_DIRS
   ]
   ```
3. Add the plugin to the registry:
   ```nu
   plugin add nu_plugin_polars
   plugin list
   ```
4. Import the plugin:
   ```nu
   plugin use polars
   ```
5. Confirm it works:
   ```nu
   ls | polars into-df | describe
   ```

## Installing Plugins

### Core Plugins

Nushell ships with officially maintained plugins:
- **polars**: Fast columnar operations using DataFrames via Polars Library
- **formats**: Support for EML, ICS, INI, plist, and VCF formats
- **gstat**: Git repository status information
- **query**: SQL, XML, JSON, HTML querying support
- **inc**: Increment values or versions

To install core plugins with cargo:
```nu
[ nu_plugin_inc
  nu_plugin_polars
  nu_plugin_gstat
  nu_plugin_formats
  nu_plugin_query
] | each { cargo install $in --locked } | ignore
```

### Third-party Plugins

Find third-party plugins on crates.io, Git repositories, and awesome-nu. To install from crates.io:
```nu
cargo install nu_plugin_<plugin_name> --locked
```

## Registering Plugins

To add a plugin to the registry:
```nu
plugin add ./my_plugins/nu_plugin_cool  # Linux/macOS
plugin add .\my_plugins\nu_plugin_cool.exe  # Windows
```

This runs the plugin binary, communicates via the plugin protocol, and saves plugin information to the registry file (`$nu.plugin-path`).

## Importing Plugins

Once added to the registry, plugins are automatically imported when Nu starts. To import manually:
```nu
plugin use cool
```

## Managing Plugins

List installed plugins:
```nu
plugin list
```

Check if a plugin is running:
```nu
plugin list | where name == gstat | select name is_running
```

### Plugin Lifecycle

Plugins stay running while in use and are automatically stopped after a period of inactivity (default: 10 seconds). To manually stop a plugin:
```nu
plugin stop gstat
```

### Plugin Garbage Collector

Configure via `$env.config.plugin_gc`:
```nu
$env.config.plugin_gc = {
    default: {
        enabled: true
        stop_after: 10sec
    }
    plugins: {
        gstat: { stop_after: 1min }
        inc: { stop_after: 0sec }
    }
}
```

## Removing Plugins

```nu
plugin rm gstat
```

## For Plugin Developers

Nu plugins are executables that communicate with Nu over stdin/stdout or local sockets using JSON or MessagePack encoding.

**Resources:**
- Example plugins in Nu's main repo (Rust and Python)
- Debug with stderr printing
- Use trace_nu_plugin crate for protocol tracing
- Ask questions in #plugins channel on Nu Discord
