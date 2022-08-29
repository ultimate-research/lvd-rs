use crate::*;
use std::io::{BufWriter, Seek, Write};
use std::path::Path;

use binrw::{BinWrite, WriteOptions};

impl LvdFile {
    const MAGIC: &'static [u8] = b"\x00\x00\x00\x01\x0D\x01\x4C\x56\x44\x31";

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), binrw::Error> {
        let mut file = BufWriter::new(std::fs::File::create(path.as_ref())?);

        self.write(&mut file)
    }

    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> Result<(), binrw::Error> {
        (
            Self::MAGIC,
            &self.collisions,
            &self.spawns,
            &self.respawns,
            &self.camera_boundary,
            &self.blast_zone,
            (&self.enemy_generators, &self.unk1, &self.unk2, &self.unk3),
            &self.fs_area_cam,
            &self.fs_cam_limit,
            &self.damage_shapes,
            &self.item_spawners,
            &self.ptrainer_ranges,
            &self.ptrainer_platforms,
            &self.general_shapes,
            &self.general_points,
            (&self.unk4, &self.unk5, &self.unk6, &self.unk7),
            &self.shrunken_camera_boundary,
            &self.shrunken_blast_zone,
        )
            .write_options(writer, &binrw::WriteOptions::new(binrw::Endian::Big), ())
    }
}

impl<T: BinWrite<Args = ()> + BinRead<Args = ()>> BinWrite for Section<T> {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (1u8, self.data.len() as u32, &self.data).write_options(writer, options, ())
    }
}

impl BinWrite for UnsupportedSection {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (1u8, 0u32).write_options(writer, options, ())
    }
}

struct LvdList<'a, T>(&'a Vec<T>);

impl<'a, T: BinWrite<Args = ()>> BinWrite for LvdList<'a, T> {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            first.write_options(writer, options, ())?;

            for item in iter {
                1u8.write_options(writer, options, ())?;
                item.write_options(writer, options, ())?;
            }
        }

        Ok(())
    }
}

impl BinWrite for CollisionMaterial {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (self.line_material as u32, 0u32, &self.line_flags).write_options(writer, options, ())
    }
}

impl BinWrite for Collision {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (
            (
                b"\x04\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
                &self.entry,
                &self.col_flags,
            ),
            1u8,
            self.vertices.len() as u32,
            1u8,
            LvdList(&self.vertices),
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
        )
            .write_options(writer, options, ())
    }
}

impl BinWrite for CollisionCliff {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (
            b"\x03\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            &self.pos,
            &self.angle,
            &self.line_index,
        )
            .write_options(writer, options, ())
    }
}

impl BinWrite for UnknownEntry {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (
            b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            self.unk,
            1u8,
            string40(&self.string),
            &self.unk2,
            &self.unk3,
            &self.unk4,
        )
            .write_options(writer, options, ())
    }
}

#[derive(BinWrite)]
struct String38<'a> {
    #[bw(map(cstr), pad_size_to(0x38))]
    s: &'a str,
}

fn string38(s: &str) -> String38 {
    String38 { s }
}

#[derive(BinWrite)]
struct String40<'a> {
    #[bw(map(cstr), pad_size_to(0x40))]
    s: &'a str,
}

fn string40(s: &str) -> String40 {
    String40 { s }
}

fn cstr(s: &&str) -> Vec<u8> {
    s.bytes().chain(std::iter::once(0u8)).collect()
}

pub(crate) fn c_bool(&x: &bool) -> u8 {
    if x {
        1
    } else {
        0
    }
}

impl BinWrite for LvdEntry {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
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
        )
            .write_options(writer, options, ())
    }
}

impl BinWrite for Spawn {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (
            b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            &self.pos,
        )
            .write_options(writer, options, ())
    }
}

impl BinWrite for Bounds {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (
            b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            self.left,
            self.right,
            self.top,
            self.bottom,
        )
            .write_options(writer, options, ())
    }
}

impl BinWrite for ItemSpawner {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (
            b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            self.id,
            self.unk,
            1u8,
            self.sections.len() as u32,
        )
            .write_options(writer, options, ())?;

        if !self.sections.is_empty() {
            1u8.write_options(writer, options, ())?;
        }

        LvdList(&self.sections).write_options(writer, options, ())
    }
}

impl BinWrite for LvdShape {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        match self {
            Self::Point { x, y } => (b"\x03\0\0\0\x01", x, y, [0u8; 8], 1u8, 1u8, 0u32)
                .write_options(writer, options, ()),
            Self::Circle { x, y, radius } => {
                (b"\x03\0\0\0\x02", x, y, radius, [0u8; 4], 1u8, 1u8, 0u32).write_options(
                    writer,
                    options,
                    (),
                )
            }
            Self::Rectangle {
                left,
                right,
                bottom,
                top,
            } => (b"\x03\0\0\0\x03", left, right, bottom, top, 1u8, 1u8, 0u32).write_options(
                writer,
                options,
                (),
            ),
            Self::Path { points } => (
                b"\x03\0\0\0\x04",
                [0u8; 0x10],
                1u8,
                1u8,
                points.len() as u32,
                1u8,
                LvdList(points),
            )
                .write_options(writer, options, ()),
            _ => unreachable!(),
        }
    }
}

impl BinWrite for PokemonTrainerRange {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (
            b"\x04\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            &self.boundary_min,
            1u8,
            &self.boundary_max,
            1u8,
            self.trainers.len() as u32,
        )
            .write_options(writer, options, ())?;

        if !self.trainers.is_empty() {
            1u8.write_options(writer, options, ())?;
        }

        (
            LvdList(&self.trainers),
            1u8,
            string40(&self.platform_name),
            1u8,
            string40(&self.sub_name),
        )
            .write_options(writer, options, ())
    }
}

impl BinWrite for PokemonTrainerPlatform {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (
            b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            &self.pos,
        )
            .write_options(writer, options, ())
    }
}

impl BinWrite for Point {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (
            b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            self.id,
            1u8,
            self.ty,
            &self.pos,
            [0u8; 0x10],
        )
            .write_options(writer, options, ())
    }
}

impl BinWrite for DamageShape {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (
            b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            self.unk1,
            self.unk2,
            0u8,
        )
            .write_options(writer, options, ())
    }
}

impl BinWrite for GeneralShape {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        (
            b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02",
            &self.entry,
            1u8,
            self.unk1,
            &self.shape,
        )
            .write_options(writer, options, ())
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
