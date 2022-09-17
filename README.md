# particle

An unopinionated monorepo package manager for JS based applications.

## Getting started

### Installing particle

#### With MacOS

```sh
brew tap brianzchen/particle
brew install particle
# To update to latest version
brew upgrade particle
```

### Setup project

In the root of the project, create a `particle.config.json` and ensure the `workspaces` field is populated.

Now you can start using particle starting with `particle check` to create a dependency lock file. You can read our usage guide to learn more about `check` and other possible commands.

## Design

### particle.config.json

Living in the root of your project to manage configuration and custom scripts for particle.

Unlike other monorepo solutions, particle does not take advantage of a root `package.json`, instead opting for a custom config.

```js
{
  /**
   * List of globs to find directories of packages to manage dependencies for
   */
  "workspaces": Array<string>,
  /**
   * Map of custom scripts to conveniently run with particle using the `run` command
   */
  "scripts": {
    [key: string]: string,
  },
  /**
   * Options to modify the default behaviour of particle in the project
   *
   * Can be to enable features or modify defaults
   */
  "options": {
    /**
     * Whether `check` command should install dependencies to the cache
     * as well as all workspace packages
     *
     * Default: false
     */
    "check_installs": boolean,
    /**
     * Whether all/some dependencies should be the same across all packages
     * in the project.
     *
     * Default: true
     */
    "sync_dependencies": boolean | Array<string>,
  }
}
```

This allows a monorepo to not strictly run with JS based tooling and means you aren't expected to have a root `package.json` file; although particle will still scan a root `package.json` if one exists.

Normally, many tools expect to be installed and executed from the root of the project. But they also generally provide ways to be executed from a separate directory. For tools like this, we instead recommend you create one or many workspaces to run tools against the root or some other directory.

Though unnatural at first, it actually enforces a natural separation of concerns between the project tooling and application itself, where otherwise, tooling dependencies would all live in the root package.json as a `"devDependency"`.

### Dependency cache

Unlike most JS monorepo frameworks, particle does not install dependencies into a project's root `node_modules` directory. Instead, dependencies when installed go into cache dir on the user's machine first before being copied into the queried workspace.

```
registry -> local cache -> copied into queried workspace
```

Doing this yields a couple of key benefits:

- If you don't work across the whole monorepo you'll see far fewer dependencies and much faster installs
- Dependencies and their versions are locked in time by their distribution location. With a global cache, you can install a dependency either as an experiment or in another project and have it install also instantly next time
- Because all dependencies are installed within a workspace you won't experience issues with node or tools not being aware of a monorepo and not being able to resolve a package because of hoisted dependencies while still being able to keep all dependency versions in sync unless `sync_dependencies` is disabled
- By using a global cache, we won't incur penalties related to installing duplicate dependencies across the monorepo as long if they've been installed in the past
- With each workspace encapsulating their own dependencies, deployments don't need to build/install everything in the project, just the workspace itself plus all it's dependents

### <particle-root>

Whenever `package.json` scripts are called with particle, they will be called under the context of the workspace that owns it. This means that scripts execute in the workspace's directory.

This makes it easy to reason about while developing in a given workspace, if you have a script like, `"test": "./scripts/run-unit.sh"` you know it's relative to the workspace.

But sometimes, particularly with project tooling, you may need to reference the project root to find some global config. To allow workspace locales to be better abstracted, particle allows scripts in `package.json` or `particle.config.json` to embed the `<particle-root>` keyword instead of trying to relative path back up to the project root which gets replaced during runtime with the root absolute directory path.

You would use it like, `"build": "./build-script.sh --babel=<particle-root>/babel.config.json"`

## Usage

Commands can be executed anywhere inside particle project using `particle ` followed by one of the following commands.

### `check`

This is the first command that's run after setting up or cloning a particle project, as well as future rechecks. `check` will go through all workspaces and their dependencies to validate or generate the `particle.lock.json` which is a file to ensures dependencies are locked and predictable between project collaborators and regardless of when dependencies are installed.

**`check` will not install any dependencies for you though**. Because particle is designed for large scale monorepo's, it makes an assumption that the mass dependencies across your projects will be a burden to install. Instead, particle relies on other commands to infer when you want to start using a particular slice of a monorepo, at which point their dependencies and it's internal dependent's dependencies will be installed.

#### Force install

If you believe your project has not reached a scale to take advantage of this lazy install you can the `check_installs` option to `true` in the config file.

Alternatively use the `--install` flag if you want to install all dependencies as a once off. Such as in CI.

#### Rechecks

When this runs on projects with an existing `particle.lock.json`, particle will go through workspaces and the lock file to determine whether they match. If not, unless `check_installs` as has been enabled, workspace's that have mismatching dependencies will have an `outdated` file added discretely in `node_modules` which particle will check for to trigger reinstalls when future commands are run.

Dependencies mappings can change quite regularly, especially when checking out different branches or pulling the latest commit in a git project. For cases like this you may consider adding `check` as part of your post checkout hook.

#### Lock file updates

It's important to note that particle will always take an optimistic approach to resolved dependencies when updating. This means that particle will always attempt to reduce the overall number of dependency versions across a given project.

Putting this into practice, say a project allows multiple versions of `react`. One workspace updates from `^16.8.0` to `^17.0.0` while another stays at `^16.8.0`, both `16.x.x.` and `17.x.x` will exist in the lock file and different versions of react will be installed to either workspace when operated upon.

Next, if the second package some time later decides to upgrade react also, but doesn't check the version of the first package and takes `^17.1.0`, if the current locked version of `17.x.x` does not satisfy `^17.1.0` then the lock file will be modified to have a single `17.x.x` version that is compatible with `^17.1.0` given that `17.1.0` satisfies `^17.0.0` with the rules of [semver](https://semver.org/).

In summary, rechecks will always update the lock file to minimum number of versions that satisfy all ranges across workspaces and their dependencies. This means that any recheck targeting a single workspace or otherwise can have an unexpected change across other workspaces if dependencies break semver conventions.

Building this enforcement, though dramatic, helps to keep the all workspaces across a project streamlined and ensure workspace owners make more intentional choices around their accepted dependency ranges (`~`, `^`).

### `run [script]`

Run a script listed in the config file's `scripts` key, appending any additional parameters to the script.

### `workspace [workspace]`

#### `install`

`particle workspace @scope/workspace install`

#### `run [script]`

#### `[path]`

#### `root` workspace

### `workspace [script]`

Loops through every workspace and runs the specified script if available.

### `uncache [package]`

Removes a package from a user's local particle cache or all packages if no package was specified.

### `upgrade [package] [workspace]`

Upgrades a given third party dependency either across the whole project or for a single workspace if given. This updates dependency references in `package.json`(s) and runs `check` which updates the lock file.
