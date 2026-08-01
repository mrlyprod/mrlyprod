import mrlymath as mp

def test_carpet_dims():
    cell = mp.two.carpet(3, 3)
    assert cell.width == 27
    assert cell.height == 27

def test_png_magic():
    data = mp.two.carpet(3, 1).png(2)
    assert data[:4] == b"\x89PNG"

def test_seeded_noise_reproduces():
    mp.seed(7)
    a = mp.two.noise(3, 2, 0.5).to_lists()
    mp.seed(7)
    b = mp.two.noise(3, 2, 0.5).to_lists()
    assert a == b

def test_sponge_census():
    cell = mp.three.carpet(3, 1)
    assert (cell.width, cell.height, cell.depth) == (3, 3, 3)
    assert cell.volume() == 20
    assert cell.surface() == 72

def test_mandelbrot_grid():
    grid = mp.fractal.mandelbrot(8, 6, 50)
    assert len(grid) == 6
    assert all(len(row) == 8 for row in grid)
    assert all(0 <= v <= 50 for row in grid for v in row)
    assert any(v == 50 for row in grid for v in row)

def test_julia_grid():
    grid = mp.fractal.julia(8, 6, -0.4, 0.6, 50)
    assert len(grid) == 6
    assert all(len(row) == 8 for row in grid)
