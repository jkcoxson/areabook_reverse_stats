# Areabook Reverse Engineering

So I was tasked with crunching some averages from the areabooks in the mission.
Along the way, we made no friends but did reverse engineer Areabook (PMG app).
This repo serves as a reference for the Areabook API as of November 2024.
There's a lot of interesting info (and strange design choices) so enjoy.

## What does it do?

Again, you probably don't want to actually run this code, but it gets the
average time between contacts per area. The code is in a messy state, but
would be easy to modify for what you need.

## How to reverse engineer Areabook?

1. Download Android Studio
2. Create an emulator without Google Play API
3. Find the Areabook apk in x86_64 format
4. Install that apk on the device
5. Download HTTP Toolkit
6. Start doing things and look at the results
7. Copy the requests in code

## Disclaimer

Don't use this code. Just a friendly reminder that the missionary department
doesn't like missionaries writing their own code. They should make better tools,
yet here we are.

## License

MIT
