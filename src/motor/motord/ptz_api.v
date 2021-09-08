module main

struct Positions {
	mut:
		pan int
		tilt int
}

struct StepPerDegree {
	pan f32
	tilt f32
}

struct PtzApi {
	raw_api RawPtzApi
	positions Positions
	steps_per_degree StepPerDegree
}

fn (api PtzApi) calibrate() {
	api.positions.pan = 180
	api.positions.tilt = 57.5
	api.raw_api.raw_pan(api.positions.pan * )
}

fn (api PtzApi) pan(degrees f32, direction Direction) {
	api.raw_api.raw_pan(degrees, direction)
}
fn (api PtzApi) tilt(degrees f32, direction Direction) {
	api.raw_api.raw_tilt(degrees, direction)
}

fn load_motor_api() ? (PtzApi) {
	api := PtzApi {
		raw_api: load_raw_api() ?
		positions: Positions{}
		steps_per_degree: StepPerDegree {
			pan: 2.4
			tilt: 2.4
		}
	}
	api.calibrate()
	return api
}