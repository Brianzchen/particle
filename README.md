# particle

Monorepo package manager for JS based applications

## Design

### particle.config.json

Living in the root of your project to manage to manage configuration and custom scripts for particle.

```
{
  "scripts": {
    "name": "command"
  }
}
```

**note** Consider other config formats, TOML, YML, etc

## Usage

Particle commands can be executed anywhere inside a project with a `particle.config.json` file.
