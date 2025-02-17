/**
 * Copyright (c) 2014-present, Facebook, Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// @Generated by gentest/gentest.rb from gentest/fixtures/YGBorderTest.html

extern crate ordered_float;
extern crate polyhorn_yoga as yoga;

use yoga::*;

#[test]
fn test_border_no_size() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_border(Edge::Left, 10 as f32);
	root.set_border(Edge::Top, 10 as f32);
	root.set_border(Edge::Right, 10 as f32);
	root.set_border(Edge::Bottom, 10 as f32);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(20 as f32, root.get_layout_width());
	assert_eq!(20 as f32, root.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(20 as f32, root.get_layout_width());
	assert_eq!(20 as f32, root.get_layout_height());
}

#[test]
fn test_border_container_match_child() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_border(Edge::Left, 10 as f32);
	root.set_border(Edge::Top, 10 as f32);
	root.set_border(Edge::Right, 10 as f32);
	root.set_border(Edge::Bottom, 10 as f32);

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_width(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_height(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(30 as f32, root.get_layout_width());
	assert_eq!(30 as f32, root.get_layout_height());

	assert_eq!(10 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(30 as f32, root.get_layout_width());
	assert_eq!(30 as f32, root.get_layout_height());

	assert_eq!(10 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());
}

#[test]
fn test_border_flex_child() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_border(Edge::Left, 10 as f32);
	root.set_border(Edge::Top, 10 as f32);
	root.set_border(Edge::Right, 10 as f32);
	root.set_border(Edge::Bottom, 10 as f32);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_flex_grow(1 as f32);
	root_child0.set_width(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(10 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(80 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(80 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(80 as f32, root_child0.get_layout_height());
}

#[test]
fn test_border_stretch_child() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_border(Edge::Left, 10 as f32);
	root.set_border(Edge::Top, 10 as f32);
	root.set_border(Edge::Right, 10 as f32);
	root.set_border(Edge::Bottom, 10 as f32);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_height(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(10 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(80 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(10 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(80 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());
}

#[test]
fn test_border_center_child() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_justify_content(Justify::Center);
	root.set_align_items(Align::Center);
	root.set_border(Edge::Start, 10 as f32);
	root.set_border(Edge::End, 20 as f32);
	root.set_border(Edge::Bottom, 20 as f32);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_width(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_height(StyleUnit::Point((10 as f32).into()));
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(40 as f32, root_child0.get_layout_left());
	assert_eq!(35 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(50 as f32, root_child0.get_layout_left());
	assert_eq!(35 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());
}
