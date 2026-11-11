{
  "commands": [
    {"id": "solid.box", "params": {"length": 60, "width": 60, "height": 10}},
    {"id": "sketch.create", "params": {"plane": {"origin": [0, 25, 10], "x_dir": [1, 0, 0], "y_dir": [0, 0, 1]}}},
    {"id": "sketch.arc.three_point", "params": {"sketch": "Sketch1", "start": [15, 0], "end": [45, 0], "through": [30, 15]}},
    {"id": "sketch.line", "params": {"sketch": "Sketch1", "points": [[45, 0], [15, 0]]}},
    {"id": "sketch.finish", "params": {"sketch": "Sketch1"}},
    {"id": "solid.extrude", "params": {"sketch": "Sketch1", "distance": 10}},
    {"id": "solid.combine", "params": {"operation": "union", "target": "Body1", "tools": ["Body2"]}}
  ]
}