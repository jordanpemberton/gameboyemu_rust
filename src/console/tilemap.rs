use crate::console::mmu::Mmu;

pub(crate) type Tile = [[u8; 8]; 8];
pub(crate) type TileMap = [[Tile; 32]; 32];

pub(crate) fn fetch_tilemap(mmu: &mut Mmu, tile_addresses: [usize; 32 * 32]) -> TileMap {
    let mut tilemap: TileMap = [[ [[0; 8]; 8]; 32]; 32];

    for row in 0..32 {
        for col in 0..32 {
            let address = tile_addresses[row * 32 + col];

            let tile_bytes: [u8; 16] = mmu.read_buffer(
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
