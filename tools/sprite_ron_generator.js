
/**
 * Tool for converting data from http://codeandweb.com/free-sprite-packer
 * to ametyst spritesheet ron file.
 */

const fs = require('fs');

const read = fs.readFileSync.bind(fs);
const write = fs.writeFileSync.bind(fs);

console.log(__dirname);
const baseDir = __dirname + '/../resources/assets/texture/';

const spritesheet = read(baseDir + 'spritesheet.json');

const data = JSON.parse(spritesheet.toString());

const getSprite = ({ frame }) => `
    (
        x: ${frame.x},
        y: ${frame.y},
        width: ${frame.w},
        height: ${frame.h},
    )
`

const getSprites = sprites => sprites.map(getSprite).join(',\n');

const generateRon = ({ meta, frames }) => `
        (
            spritesheet_width: ${meta.size.w},
            spritesheet_height: ${meta.size.h},
            sprites: [
                ${getSprites(frames)}
            ]
        )
    `;

write(baseDir + 'rpg_spritesheet.ron', generateRon(data));

// console.log(generateRon(data));