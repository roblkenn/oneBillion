const std = @import("std");
const expect = std.testing.expect;
const Thread = std.Thread;

const ONE_BILLION: usize = 1_000_000_000;

const Station = struct {
    n: u32,
    sum: f32,
    min: f32,
    max: f32,
    fn merge(self: *Station, other: *Station) void {
        self.n += other.n;
        self.sum += other.sum;
        if (other.min < self.min) {
            self.min = other.min;
        }
        if (other.max > self.max) {
            self.max = other.max;
        }
    }
};

pub fn main() !void {
    for (1..16) |number_of_threads| {
        const start_time = std.time.milliTimestamp();
        one_billion(number_of_threads);
        const end_time = std.time.milliTimestamp();
        const total_time: f64 = @as(f64, @floatFromInt(end_time - start_time)) / 1000.0;
        std.debug.print("{d} Threads with Elapsed time: {d} seconds\n\n", .{number_of_threads, total_time});
    }
}

fn one_billion(number_of_threads: usize) void {
    const partition_size: usize = ONE_BILLION / number_of_threads;
    std.debug.print("{d}\n", .{partition_size});
    for (0..number_of_threads) |partition_index| {
        std.debug.print("{d}\n", .{partition_index});
    }
}
