# particle

An unopinionated monorepo package manager for JS based applications.

## Design

### particle.config.json

Living in the root of your project to manage to manage configuration and custom scripts for particle.

Unlike other monorepo solutions, particle does not take advantage of a root `package.json` and instead using a custom config solution.

This allows a monorepo to not strictly run with JS based tooling.

<!-- consider how tools like lint-staged would work if we take this approach -->

```json
{
  "workspaces": Array<string>,
  "scripts": {
    "name": "command"
  },
  "options": {
    "checkInstalls": boolean
  }
}
```

**note** Consider other config formats, TOML, YML, etc

## Usage

Particle commands can be executed anywhere inside a project with a `particle.config.json` file.

### `check`

This is the first command that's run after setting up or cloning a particle project, as well as future rechecks. `check` will go through all workspaces and their dependencies to validate or generate the `particle.lock.json` which is a file to ensures dependencies are locked and predictable between project collaborators and regardless of when dependencies are installed.

**`check` will not install any dependencies for you though**. Because particle is designed for large scale monorepo's, it makes an assumption that the mass dependencies across your projects will be a burden to install. Instead, particle relies on other commands to infer when you want to start using a particular slice of a monorepo, at which point their dependencies and it's internal dependent's dependencies will be installed.

#### Force install

But if you believe your project has not reached a scale to take advantage of this lazy install you can the `checkInstalls` option to `true` in the config file.

Alternatively use the `--install` flag if you want to install all dependencies as a once off.

### `workspace [@scope/package] install`

### `workspace [@scope/package] run [script]`

### `workspace [@scope/package] [path]`
