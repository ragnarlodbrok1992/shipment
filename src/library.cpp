// Test function to work out how to create libraries in C++ and link them to our Rust code

extern "C" {
	__declspec(dllexport) int add(int a, int b) {
		return a + b;
	}
}
