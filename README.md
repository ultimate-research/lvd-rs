# lvd_lib

lvd_lib is a repository for libraries and tools written in Rust for working with LVD files from Super Smash Bros. for Nintendo 3DS / Wii U and Super Smash Bros. Ultimate.

## yamlvd

A command-line program for creating and editing LVD files using YAML. Drag and drop an LVD file onto the executable to create a YAML file. Drag and drop a properly structured YAML file onto the executable to create an LVD file. YAML files are text files, so they can be viewed and edited in any text editor.

Sample output from an LVD file:

```yaml
!V13
collisions: !V1
  elements:
  - !V4
    base: !V4
      meta_info: !V1
        version_info: !V1
          editor_version: 2000010101
          format_version: 2
        name: COL_00_Floor01
      dynamic_name: Ring
      dynamic_offset: !V1
        x: 0.0
        y: 0.0
        z: 0.0
      is_dynamic: true
      instance_id: 0
      instance_offset: !V1
        x: 0.0
        y: 0.0
        z: 0.0
      joint_index: -1
      joint_name: ''
    flags:
      throughable: false
      dynamic: true
    vertices: !V1
      elements:
      - !V1
        x: 41.416157
        y: -40.11807
      - !V1
        x: -41.3962
        y: -40.098976
```

### Usage

The latest executable for Windows is available in the [Releases](https://github.com/ultimate-research/lvd-rs/releases/latest).

`yamlvd <input> [output]`<br>
`yamlvd battlefield_00.lvd battlefield_00.yaml`<br>
`yamlvd battlefield_00.yaml battlefield_00.lvd`<br>
