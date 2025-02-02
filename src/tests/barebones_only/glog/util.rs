/*
Copyright 2024 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::path::Path;
use ab_glyph::{Font, PxScale};

use image::{Rgb, RgbImage};
use image_colored_text::draw::coord::DrawCoord;
use image_colored_text::text::paragraph::ColoredTextParagraph;
use image_colored_text::draw::multi_line::draw_multiline_colored_text;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

use crate::tests::barebones_only::glog::step_drawer::MY_COLOR_WHITE;



pub fn new_image_with_colored_text(path : &Path,
                                   paragraph : &ColoredTextParagraph,
                                   font: &impl Font,
                                   scale: impl Into<PxScale> + Copy) {
    //
    let (width,height,_)= paragraph.paragraph_size(scale,font);
    // ***
    let margin = 10.0;
    let img_width : f32 = 2.0*margin + width;
    let img_height : f32 = 2.0*margin + height;
    // ***
    let mut image = RgbImage::new(
        img_width as u32,
        img_height as u32
    );
    // ***
    draw_filled_rect_mut(&mut image,
                         Rect::at(0,0).of_size(img_width as u32,img_height as u32),
                         Rgb(MY_COLOR_WHITE));
    // Draw content text
    draw_multiline_colored_text(&mut image,
                                &DrawCoord::StartingAt(margin),
                                &DrawCoord::StartingAt(margin),
                                paragraph,
                                font,
                                scale);
    // ***
    image.save(path).unwrap();
}