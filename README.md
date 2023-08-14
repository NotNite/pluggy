# pluggy

Dalamud repository management tool. It lets you automatically update [the official repository](https://github.com/goatcorp/DalamudPluginsD17) along with making your own.

## Usage

> [!NOTE]
> By default, pluggy will operate in `~/.pluggy`. You can change this with the `PLUGGY_HOME` environment variable.

### Managing the official repository

First, create a config file by running `pluggy official init`. You'll answer a few questions about your Git config, and it'll clone the DalamudPluginsD17 repository for you.
