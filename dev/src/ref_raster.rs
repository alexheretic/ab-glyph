#![allow(clippy::unreadable_literal)]
use ab_glyph_rasterizer::{point, Rasterizer};

/// Draw a 16px 'w' character from DejaVuSansMono.ttf.
pub fn rasterize_ttf_w() -> Rasterizer {
    let mut rasterizer = Rasterizer::new(9, 8);
    rasterizer.draw_line(point(0.0, 0.48322153), point(1.2214766, 0.48322153));
    rasterizer.draw_line(point(1.2214766, 0.48322153), point(2.5302014, 6.557047));
    rasterizer.draw_line(point(2.5302014, 6.557047), point(3.6040268, 2.6778522));
    rasterizer.draw_line(point(3.6040268, 2.6778522), point(4.657718, 2.6778522));
    rasterizer.draw_line(point(4.657718, 2.6778522), point(5.7449665, 6.557047));
    rasterizer.draw_line(point(5.7449665, 6.557047), point(7.0536914, 0.48322153));
    rasterizer.draw_line(point(7.0536914, 0.48322153), point(8.275167, 0.48322153));
    rasterizer.draw_line(point(8.275167, 0.48322153), point(6.5167785, 8.0));
    rasterizer.draw_line(point(6.5167785, 8.0), point(5.3355703, 8.0));
    rasterizer.draw_line(point(5.3355703, 8.0), point(4.134228, 3.8791947));
    rasterizer.draw_line(point(4.134228, 3.8791947), point(2.9395974, 8.0));
    rasterizer.draw_line(point(2.9395974, 8.0), point(1.7583892, 8.0));
    rasterizer.draw_line(point(1.7583892, 8.0), point(0.0, 0.48322153));
    rasterizer.draw_line(point(0.0, 0.48322153), point(0.0, 0.48322153));
    rasterizer
}

/// Draw a 60px 'ΐ' character from OpenSans-Italic.ttf.
#[rustfmt::skip]
pub fn rasterize_ttf_iota() -> Rasterizer {
    let mut r = Rasterizer::new(14, 38);
    r.draw_quad(point(6.1964865, 34.482967), point(7.2291145, 34.482967), point(9.186806, 33.92363));
    r.draw_line(point(9.186806, 33.92363), point(9.186806, 36.69882));
    r.draw_quad(point(9.186806, 36.69882), point(8.5199, 36.978485), point(7.455002, 37.204376));
    r.draw_quad(point(7.455002, 37.204376), point(6.3901043, 37.430264), point(5.572607, 37.430264));
    r.draw_quad(point(5.572607, 37.430264), point(2.9695234, 37.430264), point(1.6034422, 36.150234));
    r.draw_quad(point(1.6034422, 36.150234), point(0.2373612, 34.870205), point(0.2373612, 32.33166));
    r.draw_quad(point(0.2373612, 32.33166), point(0.2373612, 31.621727), point(0.4417355, 30.470778));
    r.draw_quad(point(0.4417355, 30.470778), point(0.6461098, 29.319828), point(4.0021515, 13.421656));
    r.draw_line(point(4.0021515, 13.421656), point(7.594837, 13.421656));
    r.draw_line(point(7.594837, 13.421656), point(4.2603087, 29.25529));
    r.draw_quad(point(4.2603087, 29.25529), point(3.894586, 30.890284), point(3.894586, 31.92291));
    r.draw_quad(point(3.894586, 31.92291), point(3.894586, 34.482967), point(6.1964865, 34.482967));
    r.draw_line(point(6.1964865, 34.482967), point(6.1964865, 34.482967));
    r.draw_quad(point(5.615633, 6.645033), point(7.89602, 2.6220856), point(8.993188, 0.08354187));
    r.draw_line(point(8.993188, 0.08354187), point(13.18824, 0.08354187));
    r.draw_line(point(13.18824, 0.08354187), point(13.18824, 0.5138016));
    r.draw_quad(point(13.18824, 0.5138016), point(12.112585, 1.9982071), point(10.456078, 3.837574));
    r.draw_quad(point(10.456078, 3.837574), point(8.79957, 5.6769447), point(7.2291145, 7.1183205));
    r.draw_line(point(7.2291145, 7.1183205), point(5.615633, 7.1183205));
    r.draw_line(point(5.615633, 7.1183205), point(5.615633, 6.645033));
    r.draw_line(point(5.615633, 6.645033), point(5.615633, 6.645033));
    r.draw_quad(point(0.94729304, 7.354965), point(0.94729304, 6.300823), point(1.560416, 5.612406));
    r.draw_quad(point(1.560416, 5.612406), point(2.1735392, 4.9239845), point(3.1416278, 4.9239845));
    r.draw_quad(point(3.1416278, 4.9239845), point(4.8196487, 4.9239845), point(4.8196487, 6.645033));
    r.draw_quad(point(4.8196487, 6.645033), point(4.8196487, 7.720688), point(4.1635, 8.441376));
    r.draw_quad(point(4.1635, 8.441376), point(3.5073504, 9.162064), point(2.6898532, 9.162064));
    r.draw_quad(point(2.6898532, 9.162064), point(1.9368951, 9.162064), point(1.4420941, 8.688776));
    r.draw_quad(point(1.4420941, 8.688776), point(0.94729304, 8.215488), point(0.94729304, 7.354965));
    r.draw_line(point(0.94729304, 7.354965), point(0.94729304, 7.354965));
    r.draw_quad(point(9.617067, 7.354965), point(9.617067, 6.365364), point(10.219434, 5.6446743));
    r.draw_quad(point(10.219434, 5.6446743), point(10.8218, 4.9239845), point(11.811402, 4.9239845));
    r.draw_quad(point(11.811402, 4.9239845), point(13.489423, 4.9239845), point(13.489423, 6.645033));
    r.draw_quad(point(13.489423, 6.645033), point(13.489423, 7.699175), point(12.854787, 8.43062));
    r.draw_quad(point(12.854787, 8.43062), point(12.220151, 9.162064), point(11.381141, 9.162064));
    r.draw_quad(point(11.381141, 9.162064), point(10.628182, 9.162064), point(10.122625, 8.688776));
    r.draw_quad(point(10.122625, 8.688776), point(9.617067, 8.215488), point(9.617067, 7.354965));
    r.draw_line(point(9.617067, 7.354965), point(9.617067, 7.354965));
    r
}

/// Draw a 600px U+2623 character from DejaVuSansMono.ttf.
#[rustfmt::skip]
pub fn rasterize_ttf_biohazard() -> Rasterizer {
    let mut r = Rasterizer::new(294, 269);
    r.draw_quad(point(176.22818, 196.51007), point(188.30872, 189.21141), point(195.22986, 175.74664));
    r.draw_quad(point(195.22986, 175.74664), point(202.151, 162.28189), point(200.89261, 148.4396));
    r.draw_quad(point(200.89261, 148.4396), point(207.43623, 145.92282), point(213.97986, 144.9161));
    r.draw_quad(point(213.97986, 144.9161), point(216.24495, 165.5537), point(204.41609, 184.55537));
    r.draw_quad(point(204.41609, 184.55537), point(192.58723, 203.55705), point(177.48657, 210.10068));
    r.draw_quad(point(177.48657, 210.10068), point(174.46643, 203.30537), point(176.22818, 196.51007));
    r.draw_line(point(176.22818, 196.51007), point(176.22818, 196.51007));
    r.draw_quad(point(173.20804, 104.89934), point(160.87582, 98.104034), point(145.77516, 98.73323));
    r.draw_quad(point(145.77516, 98.73323), point(130.67448, 99.36243), point(119.60066, 107.164444));
    r.draw_quad(point(119.60066, 107.164444), point(113.81207, 103.13759), point(110.03691, 98.104034));
    r.draw_quad(point(110.03691, 98.104034), point(126.647644, 85.52014), point(148.92113, 86.40102));
    r.draw_quad(point(148.92113, 86.40102), point(171.19463, 87.28189), point(184.53355, 97.09732));
    r.draw_quad(point(184.53355, 97.09732), point(180.00334, 102.88591), point(173.20804, 104.89934));
    r.draw_line(point(173.20804, 104.89934), point(173.20804, 104.89934));
    r.draw_quad(point(95.69127, 152.71812), point(95.69127, 166.56041), point(103.744965, 179.39598));
    r.draw_quad(point(103.744965, 179.39598), point(111.79865, 192.23154), point(124.38254, 198.02014));
    r.draw_quad(point(124.38254, 198.02014), point(123.879196, 204.81543), point(121.11073, 210.85571));
    r.draw_quad(point(121.11073, 210.85571), point(102.48657, 202.55034), point(91.66443, 182.79362));
    r.draw_quad(point(91.66443, 182.79362), point(80.84228, 163.03693), point(82.8557, 146.67786));
    r.draw_quad(point(82.8557, 146.67786), point(90.40604, 147.68457), point(95.69127, 152.71812));
    r.draw_line(point(95.69127, 152.71812), point(95.69127, 152.71812));
    r.draw_quad(point(262.302, 253.38927), point(242.9228, 268.23825), point(213.22482, 268.23825));
    r.draw_quad(point(213.22482, 268.23825), point(204.91945, 268.23825), point(196.11073, 266.97986));
    r.draw_quad(point(196.11073, 266.97986), point(166.66443, 260.68793), point(150.30536, 230.2349));
    r.draw_line(point(150.30536, 230.2349), point(150.05368, 230.48657));
    r.draw_quad(point(150.05368, 230.48657), point(139.23154, 246.34229), point(117.83892, 258.6745));
    r.draw_quad(point(117.83892, 258.6745), point(101.22818, 266.97986), point(84.11409, 266.97986));
    r.draw_quad(point(84.11409, 266.97986), point(61.96644, 266.97986), point(39.063755, 253.64095));
    r.draw_quad(point(39.063755, 253.64095), point(64.73489, 265.72147), point(88.0151, 257.4161));
    r.draw_quad(point(88.0151, 257.4161), point(111.295296, 249.11073), point(121.86577, 231.74496));
    r.draw_quad(point(121.86577, 231.74496), point(136.71475, 202.55034), point(125.38925, 169.32886));
    r.draw_line(point(125.38925, 169.32886), point(137.72147, 161.77853));
    r.draw_quad(point(137.72147, 161.77853), point(149.802, 170.33557), point(160.1208, 162.03021));
    r.draw_line(point(160.1208, 162.03021), point(171.69798, 169.07718));
    r.draw_quad(point(171.69798, 169.07718), point(160.1208, 199.78189), point(176.98322, 226.71141));
    r.draw_quad(point(176.98322, 226.71141), point(188.05704, 247.09732), point(207.81375, 254.52182));
    r.draw_quad(point(207.81375, 254.52182), point(227.57047, 261.94632), point(262.302, 253.38927));
    r.draw_line(point(262.302, 253.38927), point(262.302, 253.38927));
    r.draw_quad(point(181.51341, 0.45303345), point(210.45636, 12.533569), point(227.06711, 51.29196));
    r.draw_quad(point(227.06711, 51.29196), point(235.87582, 79.73155), point(217.75502, 109.17786));
    r.draw_line(point(217.75502, 109.17786), point(218.25838, 109.17786));
    r.draw_quad(point(218.25838, 109.17786), point(237.13422, 110.939606), point(258.52682, 123.02014));
    r.draw_quad(point(258.52682, 123.02014), point(293.51004, 146.1745), point(293.51004, 192.23154));
    r.draw_line(point(293.51004, 192.23154), point(293.51004, 193.99329));
    r.draw_quad(point(293.51004, 193.99329), point(291.24496, 166.05705), point(272.49496, 149.698));
    r.draw_quad(point(272.49496, 149.698), point(253.74496, 133.33893), point(233.61073, 133.33893));
    r.draw_quad(point(233.61073, 133.33893), point(200.89261, 135.10068), point(177.48657, 161.52686));
    r.draw_line(point(177.48657, 161.52686), point(165.15436, 154.2282));
    r.draw_quad(point(165.15436, 154.2282), point(166.41275, 139.63087), point(153.82886, 134.849));
    r.draw_line(point(153.82886, 134.849), point(153.82886, 121.51007));
    r.draw_quad(point(153.82886, 121.51007), point(186.29529, 115.97316), point(201.39597, 88.03693));
    r.draw_quad(point(201.39597, 88.03693), point(213.4765, 67.902695), point(210.07884, 47.139267));
    r.draw_quad(point(210.07884, 47.139267), point(206.6812, 26.375854), point(181.51341, 0.45303345));
    r.draw_line(point(181.51341, 0.45303345), point(181.51341, 0.45303345));
    r.draw_quad(point(1.5637579, 197.01343), point(0.8087244, 192.7349), point(0.8087244, 188.45639));
    r.draw_quad(point(0.8087244, 188.45639), point(0.8087244, 161.77853), point(22.453018, 132.83557));
    r.draw_quad(point(22.453018, 132.83557), point(42.838924, 110.68793), point(77.31879, 111.69464));
    r.draw_line(point(77.31879, 111.69464), point(77.06711, 111.44296));
    r.draw_quad(point(77.06711, 111.44296), point(69.2651, 93.825516), point(68.76174, 69.412766));
    r.draw_quad(point(68.76174, 69.412766), point(71.781876, 21.090622), point(121.11073, 0.9563904));
    r.draw_quad(point(121.11073, 0.9563904), point(89.651, 19.580551), point(84.994965, 43.9933));
    r.draw_quad(point(84.994965, 43.9933), point(80.33892, 68.40605), point(90.15436, 86.27518));
    r.draw_quad(point(90.15436, 86.27518), point(108.27516, 113.45639), point(142.50334, 120.25168));
    r.draw_line(point(142.50334, 120.25168), point(142.75502, 135.10068));
    r.draw_quad(point(142.75502, 135.10068), point(129.16443, 141.14095), point(131.17784, 154.2282));
    r.draw_line(point(131.17784, 154.2282), point(119.85234, 160.77182));
    r.draw_quad(point(119.85234, 160.77182), point(98.7114, 135.35236), point(67.25167, 136.35907));
    r.draw_quad(point(67.25167, 136.35907), point(43.593956, 136.10739), point(27.486576, 149.44632));
    r.draw_quad(point(27.486576, 149.44632), point(11.379194, 162.78525), point(1.5637579, 197.01343));
    r.draw_line(point(1.5637579, 197.01343), point(1.5637579, 197.01343));
    r
}

/// Draw a 300px 'ę' character from Exo2-Light.otf.
#[rustfmt::skip]
pub fn rasterize_otf_tailed_e() -> Rasterizer {
    let mut r = Rasterizer::new(106, 183);
    r.draw_cubic(point(55.25, 0.75), point(16.75, 0.75), point(0.75, 17.0), point(0.75, 64.0));
    r.draw_cubic(point(0.75, 64.0), point(0.75, 110.5), point(16.75, 128.0), point(57.5, 128.0));
    r.draw_cubic(point(57.5, 128.0), point(66.75, 128.0), point(80.0, 127.0), point(90.75, 125.25));
    r.draw_cubic(point(90.75, 125.25), point(79.75, 135.25), point(74.0, 144.25), point(71.5, 151.75));
    r.draw_cubic(point(71.5, 151.75), point(65.25, 169.0), point(76.5, 183.0), point(105.75, 170.25));
    r.draw_line(point(105.75, 170.25), point(104.0, 163.5));
    r.draw_cubic(point(104.0, 163.5), point(87.0, 169.25), point(78.0, 164.75), point(83.25, 151.5));
    r.draw_cubic(point(83.25, 151.5), point(87.5, 140.0), point(94.5, 130.75), point(102.5, 122.25));
    r.draw_line(point(102.5, 122.25), point(103.0, 122.0));
    r.draw_line(point(103.0, 122.0), point(101.25, 111.25));
    r.draw_cubic(point(101.25, 111.25), point(89.75, 113.0), point(73.5, 114.25), point(59.5, 114.25));
    r.draw_cubic(point(59.5, 114.25), point(31.75, 114.25), point(19.5, 105.5), point(17.75, 72.25));
    r.draw_line(point(17.75, 72.25), point(77.75, 72.25));
    r.draw_cubic(point(77.75, 72.25), point(97.75, 72.25), point(106.0, 60.25), point(105.75, 38.5));
    r.draw_cubic(point(105.75, 38.5), point(105.5, 13.5), point(90.0, 0.75), point(55.25, 0.75));
    r.draw_line(point(55.25, 0.75), point(55.25, 0.75));
    r.draw_cubic(point(56.0, 14.5), point(79.5, 14.5), point(89.5, 21.75), point(89.75, 38.75));
    r.draw_cubic(point(89.75, 38.75), point(90.0, 50.5), point(86.75, 59.75), point(74.25, 59.75));
    r.draw_line(point(74.25, 59.75), point(17.5, 59.75));
    r.draw_cubic(point(17.5, 59.75), point(18.0, 25.5), point(28.0, 14.5), point(56.0, 14.5));
    r.draw_line(point(56.0, 14.5), point(56.0, 14.5));
    r
}

/// Draw a 300px 'ę' character from Exo2-Light.ttf.
#[rustfmt::skip]
pub fn rasterize_ttf_tailed_e() -> Rasterizer {
    let mut r = Rasterizer::new(106, 177);
    r.draw_quad(point(55.25, 0.75), point(81.25, 0.75), point(93.375, 10.25));
    r.draw_quad(point(93.375, 10.25), point(105.5, 19.75), point(105.75, 38.5));
    r.draw_quad(point(105.75, 38.5), point(106.0, 54.75), point(99.375, 63.5));
    r.draw_quad(point(99.375, 63.5), point(92.75, 72.25), point(77.75, 72.25));
    r.draw_line(point(77.75, 72.25), point(17.75, 72.25));
    r.draw_quad(point(17.75, 72.25), point(18.5, 88.75), point(23.125, 97.875));
    r.draw_quad(point(23.125, 97.875), point(27.75, 107.0), point(36.75, 110.625));
    r.draw_quad(point(36.75, 110.625), point(45.75, 114.25), point(59.5, 114.25));
    r.draw_quad(point(59.5, 114.25), point(70.0, 114.25), point(81.375, 113.375));
    r.draw_quad(point(81.375, 113.375), point(92.75, 112.5), point(101.25, 111.25));
    r.draw_line(point(101.25, 111.25), point(103.0, 122.0));
    r.draw_quad(point(103.0, 122.0), point(97.75, 124.0), point(89.625, 125.375));
    r.draw_quad(point(89.625, 125.375), point(81.5, 126.75), point(72.875, 127.375));
    r.draw_quad(point(72.875, 127.375), point(64.25, 128.0), point(57.5, 128.0));
    r.draw_quad(point(57.5, 128.0), point(37.0, 128.0), point(24.625, 121.625));
    r.draw_quad(point(24.625, 121.625), point(12.25, 115.25), point(6.5, 101.25));
    r.draw_quad(point(6.5, 101.25), point(0.75, 87.25), point(0.75, 64.0));
    r.draw_quad(point(0.75, 64.0), point(0.75, 40.5), point(6.375, 26.625));
    r.draw_quad(point(6.375, 26.625), point(12.0, 12.75), point(24.0, 6.75));
    r.draw_quad(point(24.0, 6.75), point(36.0, 0.75), point(55.25, 0.75));
    r.draw_line(point(55.25, 0.75), point(55.25, 0.75));
    r.draw_quad(point(56.0, 14.5), point(42.0, 14.5), point(33.625, 18.625));
    r.draw_quad(point(33.625, 18.625), point(25.25, 22.75), point(21.5, 32.625));
    r.draw_quad(point(21.5, 32.625), point(17.75, 42.5), point(17.5, 59.75));
    r.draw_line(point(17.5, 59.75), point(74.25, 59.75));
    r.draw_quad(point(74.25, 59.75), point(83.5, 59.75), point(86.75, 53.75));
    r.draw_quad(point(86.75, 53.75), point(90.0, 47.75), point(89.75, 38.75));
    r.draw_quad(point(89.75, 38.75), point(89.5, 26.0), point(81.5, 20.25));
    r.draw_quad(point(81.5, 20.25), point(73.5, 14.5), point(56.0, 14.5));
    r.draw_line(point(56.0, 14.5), point(56.0, 14.5));
    r.draw_line(point(95.25, 121.5), point(103.0, 122.0));
    r.draw_quad(point(103.0, 122.0), point(97.0, 128.5), point(91.75, 135.625));
    r.draw_quad(point(91.75, 135.625), point(86.5, 142.75), point(83.25, 151.5));
    r.draw_quad(point(83.25, 151.5), point(79.25, 161.5), point(85.375, 164.625));
    r.draw_quad(point(85.375, 164.625), point(91.5, 167.75), point(104.0, 163.75));
    r.draw_line(point(104.0, 163.75), point(106.0, 170.25));
    r.draw_quad(point(106.0, 170.25), point(91.25, 176.75), point(82.625, 175.125));
    r.draw_quad(point(82.625, 175.125), point(74.0, 173.5), point(71.25, 167.0));
    r.draw_quad(point(71.25, 167.0), point(68.5, 160.5), point(71.5, 151.75));
    r.draw_quad(point(71.5, 151.75), point(73.75, 145.5), point(79.25, 137.875));
    r.draw_quad(point(79.25, 137.875), point(84.75, 130.25), point(95.25, 121.5));
    r.draw_line(point(95.25, 121.5), point(95.25, 121.5));
    r
}
