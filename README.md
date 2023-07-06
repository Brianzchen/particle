# particle

## The vision

A monorepo manager handling just the

- organization
- orchestration

Some key pillars `particle` aims to uphold,

- The larger the codebase gets shouldn't compromise the speed of operations of the repository
- Encourage layered reuse without spaghetti library consumption
- A project within the repo can be added immediately be working autonomously and productively
- An incremental upgrade system for both internal and external libraries
- Hot fixing past deployments is totally possible
- Libraries developed in the repo can both either work towards a release to their internal consumers or have immediate consumption

## Raw ideas

Should have nothing more than a `particle.config.json` to denote that this is a particle project. This allows particle to look upwards to find the root directory before performing tasks.

Workspaces are not the same as particles. Where inside a project, you may choose to use a yarn workspace to organize internal package imports.

Though a workspace can be turned into a particle by adding the `"particle"` object field, which can be initially empty.

package management is generally the least of someone's choice of tools, so particle should handle this.

### Package manager features

deps, devDev installation

lock file

resolutions

installing via url or version

handling caret, tilde

warning of peer dependency mismatches based on range

usage of scopes and reading npmrc

Then dump the right node_modules, available in root or nested and node should function fine

### `particle.config.json`

```json
{

}
```

### `package.json`

```json
{
  ...
  "particle": {
    "layer": "atomic" | "library" | "project" // default as project
  }
}
```
