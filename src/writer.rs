use crate::*;
use std::path::Path;
use std::io::{self, Write, BufWriter};

use binwrite::{BinWrite, WriterOption};

impl LvdFile {
    const MAGIC: &'static [u8] = b"\x00\x00\x00\x01\x0D\x01\x4C\x56\x44\x31";

    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = BufWriter::new(std::fs::File::create(path.as_ref())?);
        
        self.write(&mut file)
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        (
            Self::MAGIC,
            &self.collisions,
            &self.spawns,
            &self.respawns,
            &self.camera,
            &self.blastzones,

            (
                &self.enemy_generators,
                &self.unk1,
                &self.unk2,
                &self.unk3,
            ),
            &self.fs_area_cam,
            &self.fs_cam_limit, 
            &self.damage_shapes,
            &self.item_spawners,
            &self.ptrainers,
            &self.ptrainer_platform,
            &self.general_shapes,
            &self.general_points,
            (
                &self.unk4,
                &self.unk5,
                &self.unk6,
                &self.unk7,
            ),
            &self.shrunk_cameras,
            &self.shrunk_blastzones,
        ).write_options(writer, &binwrite::writer_option_new!(endian: binwrite::Endian::Big))
    }
}

impl<T: BinWrite + BinRead<Args = ()>> BinWrite for Section<T> {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            1u8,
            self.data.len() as u32,
            &self.data
        ).write_options(writer, options)
    }
}

impl BinWrite for UnsupportedSection {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (1u8, 0u32).write_options(writer, options)
    }
}

struct LvdList<'a, T>(&'a Vec<T>);

impl<'a, T: BinWrite> BinWrite for LvdList<'a, T> {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            first.write_options(writer, options)?;

            for item in iter {
                1u8.write_options(writer, options)?;
                item.write_options(writer, options)?;
            }
        }

        Ok(())
    }
}

impl BinWrite for CollisionMaterial {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            self.line_material as u32,
            0u32,
            &self.line_flags,
        ).write_options(writer, options)
    }
}

impl BinWrite for Collision {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            (
                b"\x04\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
                &self.entry,
                &self.col_flags,
            ),
            1u8,
            self.verts.len() as u32,
            1u8,
            LvdList(&self.verts),
            1u8,
            self.normals.len() as u32,
            1u8,
            LvdList(&self.normals),
            1u8,
            self.cliffs.len() as u32,
            &self.cliffs,
            1u8,
            self.materials.len() as u32,
            1u8,
            LvdList(&self.materials),
            1u8,
            self.unknowns.len() as u32,
            &self.unknowns,
        ).write_options(writer, options)
    }
}

impl BinWrite for CollisionCliff {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            b"\x03\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            &self.pos,
            &self.angle,
            &self.line_index,
        ).write_options(writer, options)
    }
}

impl BinWrite for UnknownEntry {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            b"\x04\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            self.unk,
            1u8,
            string38(&self.string),
            &self.unk2,
            &self.unk3,
            &self.unk3,
        ).write_options(writer, options)
    }
}

#[derive(BinWrite)]
struct String38<'a> {
    #[binwrite(cstr, align_after(0x38))]
    s: &'a str
}

fn string38(s: &str) -> String38 {
    String38 { s }
}

#[derive(BinWrite)]
struct String40<'a> {
    #[binwrite(cstr, align_after(0x40))]
    s: &'a str
}

fn string40(s: &str) -> String40 {
    String40 { s }
}

pub(crate) fn c_bool(&x: &bool) -> u8 {
    if x { 1 } else { 0 }
}

impl BinWrite for LvdEntry {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            1u8,
            string38(&self.name),
            1u8,
            string40(&self.subname),
            1u8,
            &self.start_pos,
            c_bool(&self.use_start),
            1u8,
            self.unk,
            1u8,
            &self.unk2,
            self.unk3,
            1u8,
            string40(&self.bone_name),
        ).write_options(writer, options)
    }
}

impl BinWrite for Spawn {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            &self.pos,
        ).write_options(writer, options)
    }
}

impl BinWrite for Bounds {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            self.left,
            self.right,
            self.top,
            self.bottom,
        ).write_options(writer, options)
    }
}

impl BinWrite for ItemSpawner {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            self.id,
            self.unk,
            1u8,
            self.sections.len() as u32,
            1u8,
            LvdList(&self.sections),
        ).write_options(writer, options)
    }
}

impl BinWrite for LvdShape {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        match self {
            Self::Point { x, y } => (
                b"\x03\0\0\0\x01",
                x,
                y,
                [0u8; 8],
                0u8,
                0u32
            ).write_options(writer, options),
            Self::Circle { x, y, r } => (
                b"\x03\0\0\0\x02",
                x,
                y,
                r,
                [0u8; 4],
                0u8,
                0u32
            ).write_options(writer, options),
            Self::Rectangle { left, right, bottom, top } => (
                b"\x03\0\0\0\x03",
                left,
                right,
                bottom,
                top,
                0u8,
                0u32
            ).write_options(writer, options),
            Self::Path { points } => (
                b"\x03\0\0\0\x04",
                [0u8; 0x10],
                1u8,
                1u8,
                points.len() as u32,
                1u8,
                LvdList(points),
            ).write_options(writer, options),
            _ => unreachable!()
        }
    }
}

impl BinWrite for PokemonTrainer {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            b"\x04\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            &self.boundary_min,
            1u8,
            &self.boundary_max,
            1u8,
            self.trainers.len() as u32,
            1u8,
            LvdList(&self.trainers),
            1u8,
            string40(&self.platform_name),
            1u8,
            string40(&self.sub_name),
        ).write_options(writer, options)
    }
}

impl BinWrite for PokemonTrainerPlatform {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            &self.pos
        ).write_options(writer, options)
    }
}

impl BinWrite for Point {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        (
            b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            self.id,
            1u8,
            self.ty,
            &self.pos,
            [0u8; 0x10],
        ).write_options(writer, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let lvd = LvdFile::open("/home/jam/Downloads/param/pickel_world_00.lvd").unwrap();

        lvd.save("test_out.lvd").unwrap();
    }
}
