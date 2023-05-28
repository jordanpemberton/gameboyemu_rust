use crate::console::mmu::Mmu;

pub(crate) type Tile = [[u8; 8]; 8];
pub(crate) type TileMap = [[Tile; 32]; 32];

pub(crate) fn get_tilemap(mmu: &mut Mmu, tilemap_address: usize, index_mode_8000: bool) -> TileMap {
    let tile_indices = fetch_tile_indices(mmu, tilemap_address);
    let tile_addresses = fetch_tile_addresses(tile_indices, index_mode_8000);
    let tilemap = fetch_tilemap(mmu, tile_addresses);
    tilemap
}

pub(crate) fn fetch_tilemap(mmu: &mut Mmu, tile_addresses: [usize; 32 * 32]) -> TileMap {
    let mut tilemap: TileMap = [[ [[0; 8]; 8]; 32]; 32];

    for row in 0..32 {
        for col in 0..32 {
            let address = tile_addresses[row * 32 + col];

            let tile_bytes: [u8; 16] = mmu.read(
                address,
                address + 16)
                .try_into().unwrap();

            tilemap[row][col] = read_tile(tile_bytes);
        }
    }

    tilemap
}

pub(crate) fn read_tile(tile_bytes: [u8; 16]) -> Tile {
    let mut tile: Tile = [[0; 8]; 8];

    for row in 0..8 {
        for col in 0..8 {
            // Possible values = 0,1,2,3 (0b00,0b01,0b10,0b11)
            let low = ((tile_bytes[row * 2] << col) >> 7) << 1;
            let high = (tile_bytes[row * 2 + 1] << col) >> 7;
            tile[row][col] = high + low;
        }
    }

    tile
}

fn fetch_tile_indices(mmu: &mut Mmu, tilemap_address: usize) -> [usize; 32 * 32] {
    let indices: [usize; 32 * 32] = mmu.read(
        tilemap_address,
        tilemap_address + 32 * 32)
        .iter().map(|x| *x as usize)
        .collect::<Vec<usize>>()
        .try_into().unwrap();
    indices
}

fn fetch_tile_addresses(tile_indices: [usize; 32 * 32], index_mode_8000: bool) -> [usize; 32 * 32] {
    let mut addresses = [0; 32 * 32];

    for i in 0..32 * 32 {
        let tile_index = tile_indices[i] as i32;
        addresses[i] = if !index_mode_8000 && tile_index < 128 {
            0x9000 + tile_index * 16
        } else {
            0x8000 + tile_index * 16
        } as usize;
    }

    addresses
}
