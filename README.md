# pluggy

Dalamud repository management tool. It lets you automatically update [the official repository](https://github.com/goatcorp/DalamudPluginsD17) along with making your own.

> [!NOTE]
> By default, pluggy will operate in your operating system's default configuration directory. You can change this with the `PLUGGY_HOME` environment variable.

## Managing the official repository

First, create a config file by running `pluggy official init`. You'll answer a few questions about your Git config, and it'll clone the DalamudPluginsD17 repository for you.

Then, when you need to update your plugin(s), call the `update` command with the InternalName and Git repository of the plugin:

```shell
pluggy official update DistantSeas "https://github.com/NotNite/DistantSeas.git"
```

This command will clone your repository, build the plugin to get some information about it, make a new branch on your DalamudPluginsD17 fork, and make a new commit on that branch updating the manifest of your plugin.

The manifest will be completely rewritten, so any existing information in the manifest (like changelogs or owners) will be lost. Because of this, make sure to specify the information you need every time (see `pluggy official update -h`). The most common ones you may need to change are `--path`, `--owners`, and `--testing`/`--track`.

You'll need to run this exact command for every update to the plugin (including the InternalName and Git repository). This is *not* a one time operation.

pluggy builds your plugin locally to get the name and version from the output manifest. If you can't build your plugin by just running `dotnet build` (e.g. uses submodules, .NET not installed, complex build script) then you can't use pluggy right now.

Once you're ready to publish the update, call the `push` command with the InternalName of the plugin:

```shell
pluggy official push DistantSeas
```

This will push the branch with the updated manifest to your fork of DalamudPluginsD17. You can then make a PR with that branch, write a changelog, and you're done!
