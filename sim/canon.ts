import type { Ctx, Json, Obj } from "@mrlyprod/os/kernel/index.ts";
import { below, nearest } from "@mrlyprod/os/kernel/index.ts";

export type Thing = {
  id: string;
  name: string;
  kind: string;
  food?: number;
  fun?: number;
  price?: number;
  by?: string;
  about?: string;
  added?: string;
  at?: number;
};

export type Lookup = Pick<Ctx, "knob" | "read">;

type Row = { kind: string; names: string; more?: Pick<Thing, "food" | "fun" | "price">; maker?: [string, string] };

const NOT = "canon.not";

const ONLY = "canon.only";

const ADDED = "canon/added";

const NEAR = 3;

const eat = (kind: string, food: number, fun: number, price: number, names: string): Row => ({
  kind,
  names,
  more: { food, fun, price },
});

const good = (kind: string, price: number, fun: number, names: string): Row => ({ kind, names, more: { price, fun } });

const plain = (kind: string, names: string): Row => ({ kind, names });

const works = (kind: string, makers: string, table: Record<string, string>): Row[] =>
  Object.entries(table).map(([maker, names]) => ({ kind, names, maker: [maker, makers] }));

const FOODS: Row[] = [
  eat("fruit", 3, 2, 1, "Apple|Banana|Pear|Orange|Plum|Tangerine|Clementine|Apricot|Nectarine|Peach"),
  eat("fruit", 1, 1, 1, "Lemon|Lime|Grapefruit|Quince|Cranberry"),
  eat("fruit", 2, 4, 2, "Strawberry|Blueberry|Raspberry|Blackberry|Cherry|Grape|Gooseberry|Currant"),
  eat("fruit", 3, 4, 2, "Mango|Papaya|Kiwi|Fig|Date|Guava|Persimmon|Pomegranate|Lychee|Passion Fruit|Plantain|Melon"),
  eat("fruit", 4, 5, 3, "Pineapple|Watermelon|Coconut|Jackfruit|Dragon Fruit|Durian|Avocado|Starfruit"),
  eat("vegetable", 2, 1, 1, "Carrot|Onion|Garlic|Celery|Cabbage|Cucumber|Radish|Turnip|Beet|Leek|Parsnip|Lettuce"),
  eat("vegetable", 2, 1, 1, "Spinach|Kale|Chard|Arugula|Watercress|Scallion|Shallot"),
  eat("vegetable", 3, 2, 2, "Tomato|Bell Pepper|Broccoli|Cauliflower|Zucchini|Eggplant|Green Bean|Pea|Sweet Corn"),
  eat("vegetable", 3, 2, 2, "Mushroom|Asparagus|Artichoke|Okra|Brussels Sprout|Bok Choy|Fennel|Chili Pepper"),
  eat("vegetable", 5, 2, 1, "Potato|Sweet Potato|Yam|Pumpkin|Butternut Squash|Cassava|Taro"),
  eat("grain", 5, 1, 1, "Rice|Wheat|Oats|Barley|Rye|Millet|Sorghum|Corn"),
  eat("grain", 5, 1, 1, "Quinoa|Buckwheat|Couscous|Bulgur|Polenta|Farro"),
  eat("grain", 6, 2, 2, "Bread|Pasta|Noodles|Tortilla|Bagel|Baguette|Pita|Naan|Cereal"),
  eat("protein", 4, 2, 1, "Egg|Tofu|Tempeh|Lentils|Chickpeas|Black Beans|Kidney Beans|Soybeans|Peanuts|Almonds"),
  eat("protein", 4, 2, 2, "Walnuts|Cashews|Seitan"),
  eat("protein", 8, 4, 6, "Chicken|Turkey|Pork|Bacon|Sausage|Ham|Duck|Tuna|Sardines|Cod|Mackerel|Trout|Tilapia"),
  eat("protein", 9, 6, 12, "Beef|Steak|Lamb|Salmon|Shrimp|Crab|Lobster|Oyster|Scallop|Venison"),
  eat("dairy", 3, 2, 2, "Butter|Cream|Yogurt|Kefir|Cottage Cheese|Sour Cream|Ghee"),
  eat("dairy", 4, 5, 4, "Cheddar|Mozzarella|Brie|Parmesan|Feta|Gouda|Camembert|Swiss Cheese|Blue Cheese|Halloumi"),
  eat("drink", 0, 1, 1, "Water|Sparkling Water"),
  eat("drink", 1, 3, 2, "Tea|Green Tea|Coffee|Espresso|Hot Chocolate|Lemonade|Iced Tea|Orange Juice|Apple Juice"),
  eat("drink", 1, 4, 2, "Soda|Cola|Ginger Ale|Root Beer|Kombucha"),
  eat("drink", 3, 2, 2, "Milk|Oat Milk|Soy Milk|Almond Milk"),
  eat("drink", 3, 6, 4, "Smoothie|Milkshake"),
  eat("drink", 1, 7, 5, "Beer|Wine|Cider|Sake|Whisky|Vodka|Gin|Rum|Tequila|Champagne|Mead"),
  eat("snack", 2, 5, 2, "Potato Chips|Popcorn|Pretzels|Crackers|Nachos|Trail Mix|Granola Bar|Rice Crackers"),
  eat("snack", 2, 4, 3, "Beef Jerky|Seaweed Snack|Olives|Pickles|Hummus|Sunflower Seeds|Pistachios"),
  eat("meal", 7, 6, 6, "Sandwich|Soup|Salad|Omelette|Stew|Porridge|Fried Rice|Mac and Cheese|Grilled Cheese"),
  eat("meal", 8, 8, 10, "Pizza|Hamburger|Hot Dog|Taco|Burrito|Sushi|Ramen|Dumplings|Falafel|Kebab|Fried Chicken"),
  eat("meal", 8, 8, 10, "Curry|Paella|Lasagna|Pad Thai|Pho|Bibimbap|Risotto|Spaghetti Bolognese|Fish and Chips"),
  eat("meal", 8, 8, 10, "Roast Dinner|Biryani|Tagine|Shakshuka|Moussaka|Pierogi"),
  eat("dessert", 3, 8, 3, "Chocolate|Cookie|Brownie|Donut|Muffin|Croissant"),
  eat("dessert", 3, 8, 3, "Waffle|Pancake|Ice Cream|Gelato|Candy|Honey"),
  eat("dessert", 4, 9, 6, "Cheesecake|Tiramisu|Apple Pie|Creme Brulee|Baklava|Churros|Macaron|Cupcake|Mochi"),
  eat("dessert", 4, 9, 6, "Panna Cotta|Birthday Cake"),
  eat("pantry", 0, 2, 2, "Salt|Black Pepper|Sugar|Cinnamon|Cumin|Turmeric|Paprika|Ginger|Vanilla|Basil|Oregano|Thyme"),
  eat("pantry", 0, 2, 2, "Rosemary|Mint|Saffron|Nutmeg|Cardamom"),
  eat("pantry", 0, 2, 2, "Soy Sauce|Olive Oil|Vinegar|Mustard|Ketchup|Mayonnaise"),
];

const GOODS: Row[] = [
  good("household", 3, 0, "Soap|Toothpaste|Toothbrush|Shampoo|Toilet Paper|Paper Towels|Dish Soap|Laundry Detergent"),
  good("household", 3, 0, "Sponge|Trash Bags|Light Bulb|Batteries|Candle|Matches"),
  good("household", 3, 0, "Tissues|Deodorant|Razor|Comb|Hairbrush"),
  good("household", 15, 1, "Towel|Broom|Mop|Bucket|Pillow|Blanket|Bedsheets|Curtains|Doormat|Laundry Basket|Mug"),
  good("household", 15, 1, "Plate|Bowl|Fork|Kitchen Knife|Spoon|Frying Pan|Saucepan|Kettle|Cutting Board|Alarm Clock"),
  good("household", 15, 2, "Picture Frame|Vase|Houseplant"),
  good("household", 60, 2, "Toaster|Blender|Vacuum Cleaner|Microwave|Coffee Maker|Rice Cooker|Iron|Hair Dryer|Fan"),
  good("household", 60, 2, "Lamp|Mirror|Rug"),
  good("household", 300, 3, "Refrigerator|Washing Machine|Dishwasher|Oven|Bed|Sofa|Dining Table|Desk|Wardrobe"),
  good("household", 300, 3, "Bookshelf|Armchair"),
  good("clothing", 8, 1, "Socks|Underwear|T-Shirt|Hat|Scarf|Gloves|Belt|Tie|Beanie|Flip-Flops"),
  good("clothing", 30, 2, "Jeans|Shirt|Sweater|Hoodie|Skirt|Dress|Shorts"),
  good("clothing", 30, 2, "Leggings|Pajamas|Swimsuit|Sneakers|Sandals|Cap"),
  good("clothing", 80, 3, "Coat|Jacket|Raincoat|Boots|Suit|Blazer|Heels|Sunglasses|Wristwatch|Backpack|Handbag"),
  good("tool", 20, 1, "Hammer|Screwdriver|Wrench|Pliers|Saw|Drill|Tape Measure|Ladder|Shovel|Rake|Axe|Flashlight"),
  good("tool", 5, 1, "Paintbrush|Scissors|Stapler|Pen|Pencil|Notebook|Umbrella"),
  good("electronics", 300, 5, "Smartphone|Laptop|Tablet|Desktop Computer|Television|Headphones|Speaker|Camera"),
  good("electronics", 300, 5, "Game Console|Smartwatch|Printer|Router|E-Reader|Radio|Drone"),
  good("vehicle", 200, 5, "Bicycle|Scooter|Skateboard|Canoe"),
  good("vehicle", 20000, 4, "Motorcycle|Car|Truck|Van|Sailboat"),
  good("instrument", 200, 7, "Piano|Guitar|Violin|Cello|Flute|Clarinet|Saxophone|Trumpet|Trombone|Drums|Harp|Ukulele"),
  good("instrument", 200, 7, "Banjo|Accordion|Harmonica|Bass Guitar|Oboe|Sitar|Tabla|Organ"),
  good("game", 30, 8, "Chess|Go|Checkers|Monopoly|Scrabble|Backgammon|Poker|Bridge|Mahjong|Dominoes|Risk|Catan|Clue"),
  good("game", 30, 8, "Tetris|Minecraft|Pac-Man|Super Mario Bros.|The Legend of Zelda|Pokémon|The Sims|Doom"),
  good("game", 30, 8, "Grand Theft Auto|Fortnite|Portal|Street Fighter|Sudoku"),
];

const MODELS: Record<string, string> = {
  Anthropic: "Claude",
  OpenAI: "GPT",
  Google: "Gemini|Gemma",
  "DeepSeek AI": "DeepSeek",
  Meta: "Llama",
  "Mistral AI": "Mistral",
  xAI: "Grok",
  Alibaba: "Qwen",
  Microsoft: "Phi",
  Cohere: "Command",
  Amazon: "Nova",
  "Moonshot AI": "Kimi",
  "AI21 Labs": "Jamba",
  Nvidia: "Nemotron",
  "Zhipu AI": "GLM",
};

const BOOKS: Record<string, string> = {
  "Oscar Wilde":
    "The Picture of Dorian Gray|The Importance of Being Earnest|An Ideal Husband|Lady Windermere's Fan|A Woman of No Importance|Salome|The Happy Prince|De Profundis|The Ballad of Reading Gaol",
  "Jane Austen": "Pride and Prejudice|Sense and Sensibility|Emma|Persuasion|Mansfield Park|Northanger Abbey",
  "Charles Dickens":
    "Great Expectations|Oliver Twist|A Tale of Two Cities|David Copperfield|A Christmas Carol|Bleak House",
  "Leo Tolstoy": "War and Peace|Anna Karenina|The Death of Ivan Ilyich",
  "Fyodor Dostoevsky": "Crime and Punishment|The Brothers Karamazov|The Idiot|Notes from Underground",
  "Mark Twain": "The Adventures of Tom Sawyer|Adventures of Huckleberry Finn|The Prince and the Pauper",
  "Herman Melville": "Moby-Dick|Bartleby, the Scrivener",
  "Mary Shelley": "Frankenstein|The Last Man",
  "Bram Stoker": "Dracula",
  "Charlotte Brontë": "Jane Eyre|Villette",
  "Emily Brontë": "Wuthering Heights",
  "George Eliot": "Middlemarch|Silas Marner|The Mill on the Floss",
  Homer: "The Iliad|The Odyssey",
  "William Shakespeare": "Hamlet|Macbeth|Romeo and Juliet|King Lear|Othello|A Midsummer Night's Dream|The Tempest",
  "Dante Alighieri": "The Divine Comedy",
  "Miguel de Cervantes": "Don Quixote",
  "Lewis Carroll": "Alice's Adventures in Wonderland|Through the Looking-Glass",
  "Arthur Conan Doyle": "A Study in Scarlet|The Hound of the Baskervilles|The Sign of the Four",
  "Agatha Christie":
    "Murder on the Orient Express|And Then There Were None|Death on the Nile|The Murder of Roger Ackroyd",
  "F. Scott Fitzgerald": "The Great Gatsby|Tender Is the Night",
  "Ernest Hemingway": "The Old Man and the Sea|A Farewell to Arms|The Sun Also Rises",
  "George Orwell": "Nineteen Eighty-Four|Animal Farm|Homage to Catalonia",
  "Aldous Huxley": "Brave New World|The Doors of Perception",
  "Virginia Woolf": "Mrs Dalloway|To the Lighthouse|Orlando|A Room of One's Own",
  "James Joyce": "Ulysses|Dubliners|A Portrait of the Artist as a Young Man",
  "Franz Kafka": "The Metamorphosis|The Trial|The Castle",
  "J. R. R. Tolkien": "The Hobbit|The Lord of the Rings|The Silmarillion",
  "C. S. Lewis": "The Lion, the Witch and the Wardrobe|Prince Caspian|The Screwtape Letters",
  "J. K. Rowling":
    "Harry Potter and the Philosopher's Stone|Harry Potter and the Chamber of Secrets|Harry Potter and the Prisoner of Azkaban",
  "Harper Lee": "To Kill a Mockingbird|Go Set a Watchman",
  "J. D. Salinger": "The Catcher in the Rye|Franny and Zooey",
  "Gabriel García Márquez": "One Hundred Years of Solitude|Love in the Time of Cholera",
  "Toni Morrison": "Beloved|Song of Solomon|The Bluest Eye",
  "Chinua Achebe": "Things Fall Apart|No Longer at Ease",
  "Haruki Murakami": "Norwegian Wood|Kafka on the Shore|The Wind-Up Bird Chronicle",
  "Margaret Atwood": "The Handmaid's Tale|Oryx and Crake",
  "Stephen King": "The Shining|It|Carrie|Misery",
  "Frank Herbert": "Dune|Dune Messiah",
  "Isaac Asimov": "Foundation|I, Robot",
  "Ursula K. Le Guin": "A Wizard of Earthsea|The Left Hand of Darkness|The Dispossessed",
  "Douglas Adams": "The Hitchhiker's Guide to the Galaxy|Dirk Gently's Holistic Detective Agency",
  "Terry Pratchett": "The Colour of Magic|Mort|Going Postal",
  "Roald Dahl": "Charlie and the Chocolate Factory|Matilda|The BFG",
  "Dr. Seuss": "The Cat in the Hat|Green Eggs and Ham",
  "Walt Whitman": "Leaves of Grass",
  "Sun Tzu": "The Art of War",
  "Marcus Aurelius": "Meditations",
  Plato: "The Republic|Symposium",
  "Niccolò Machiavelli": "The Prince",
  "Charles Darwin": "On the Origin of Species|The Descent of Man",
  "Adam Smith": "The Wealth of Nations",
  "Stephen Hawking": "A Brief History of Time",
  "Yuval Noah Harari": "Sapiens",
  "Anne Frank": "The Diary of a Young Girl",
  "Victor Hugo": "Les Misérables|The Hunchback of Notre-Dame",
  "Alexandre Dumas": "The Three Musketeers|The Count of Monte Cristo",
  "Jules Verne": "Twenty Thousand Leagues Under the Seas|Around the World in Eighty Days",
  "H. G. Wells": "The War of the Worlds|The Time Machine|The Invisible Man",
  "Kurt Vonnegut": "Slaughterhouse-Five|Cat's Cradle",
  "Ray Bradbury": "Fahrenheit 451|The Martian Chronicles",
  "Cormac McCarthy": "The Road|Blood Meridian",
  "Kazuo Ishiguro": "The Remains of the Day|Never Let Me Go",
  "Zadie Smith": "White Teeth",
  "Paulo Coelho": "The Alchemist",
  "Antoine de Saint-Exupéry": "The Little Prince",
  "Albert Camus": "The Stranger|The Plague",
  "Suzanne Collins": "The Hunger Games|Catching Fire",
};

const FILMS: Record<string, string> = {
  "James Cameron": "Titanic|Avatar|The Terminator|Aliens|The Abyss",
  "Steven Spielberg": "Jaws|E.T. the Extra-Terrestrial|Jurassic Park|Schindler's List|Raiders of the Lost Ark",
  "George Lucas": "Star Wars|American Graffiti",
  "Francis Ford Coppola": "The Godfather|Apocalypse Now",
  "Martin Scorsese": "Taxi Driver|Goodfellas|Raging Bull",
  "Stanley Kubrick": "2001: A Space Odyssey|The Shining|A Clockwork Orange|Dr. Strangelove",
  "Alfred Hitchcock": "Psycho|Vertigo|Rear Window|The Birds|North by Northwest",
  "Orson Welles": "Citizen Kane|Touch of Evil",
  "Akira Kurosawa": "Seven Samurai|Rashomon|Ran",
  "Hayao Miyazaki": "Spirited Away|My Neighbor Totoro|Princess Mononoke|Howl's Moving Castle",
  "Quentin Tarantino": "Pulp Fiction|Kill Bill|Reservoir Dogs",
  "Christopher Nolan": "Inception|The Dark Knight|Interstellar|Oppenheimer|Memento",
  "Peter Jackson": "The Lord of the Rings",
  "Ridley Scott": "Alien|Blade Runner|Gladiator|The Martian",
  "Denis Villeneuve": "Dune|Arrival|Blade Runner 2049",
  "Bong Joon-ho": "Parasite|Snowpiercer|Okja|Memories of Murder",
  "Wes Anderson": "The Grand Budapest Hotel|Moonrise Kingdom|Rushmore",
  "The Coen Brothers": "Fargo|No Country for Old Men|The Big Lebowski",
  "The Wachowskis": "The Matrix",
  "Robert Zemeckis": "Back to the Future|Forrest Gump",
  "Greta Gerwig": "Lady Bird|Little Women|Barbie",
  "Jordan Peele": "Get Out|Us|Nope",
  "Victor Fleming": "Gone with the Wind|The Wizard of Oz",
  "Michael Curtiz": "Casablanca",
  "Federico Fellini": "La Dolce Vita|La Strada",
  "Ingmar Bergman": "The Seventh Seal|Persona",
  "John Lasseter": "Toy Story|A Bug's Life|Cars",
  "Pete Docter": "Up|Inside Out|Monsters, Inc.",
  "Andrew Stanton": "Finding Nemo|WALL-E",
  "Roger Allers": "The Lion King",
  "Jennifer Lee": "Frozen",
  "Damien Chazelle": "La La Land|Whiplash",
  "Sofia Coppola": "Lost in Translation|The Virgin Suicides",
  "Spike Lee": "Do the Right Thing|Malcolm X",
  "Charlie Chaplin": "Modern Times|The Great Dictator|City Lights",
  "Billy Wilder": "Some Like It Hot|Sunset Boulevard",
  "Ang Lee": "Crouching Tiger, Hidden Dragon|Brokeback Mountain|Life of Pi",
  "David Lynch": "Mulholland Drive|Blue Velvet",
  "Frank Darabont": "The Shawshank Redemption|The Green Mile",
};

const SHOWS: Record<string, string> = {
  "Vince Gilligan": "Breaking Bad|Better Call Saul",
  "David Chase": "The Sopranos",
  "David Simon": "The Wire|Treme",
  "David Benioff": "Game of Thrones",
  "Matt Groening": "The Simpsons|Futurama",
  "Trey Parker": "South Park",
  "Seth MacFarlane": "Family Guy",
  "Larry David": "Seinfeld|Curb Your Enthusiasm",
  "David Crane": "Friends",
  "Greg Daniels": "The Office|Parks and Recreation",
  "Phoebe Waller-Bridge": "Fleabag",
  "Hwang Dong-hyuk": "Squid Game",
  "The Duffer Brothers": "Stranger Things",
  "Charlie Brooker": "Black Mirror",
  "Jesse Armstrong": "Succession",
  "Shonda Rhimes": "Grey's Anatomy|Scandal",
  "Rod Serling": "The Twilight Zone",
  "Gene Roddenberry": "Star Trek",
  "Sydney Newman": "Doctor Who",
  "Peter Morgan": "The Crown",
  "Julian Fellowes": "Downton Abbey",
  "David Lynch": "Twin Peaks",
  "Dan Harmon": "Community|Rick and Morty",
  "Joss Whedon": "Buffy the Vampire Slayer|Firefly",
  "J. J. Abrams": "Lost|Alias",
  "Jim Henson": "The Muppet Show",
  "Steven Knight": "Peaky Blinders",
  "Bill Lawrence": "Scrubs|Ted Lasso",
  "Mike Judge": "Beavis and Butt-Head|King of the Hill",
};

const SONGS: Record<string, string> = {
  "The Beatles": "Hey Jude|Let It Be|Yesterday|Come Together|Here Comes the Sun",
  "The Rolling Stones": "Paint It Black|Gimme Shelter|Satisfaction",
  Queen: "Bohemian Rhapsody|Don't Stop Me Now|Another One Bites the Dust",
  "David Bowie": "Space Oddity|Heroes|Life on Mars?",
  "Bob Dylan": "Like a Rolling Stone|Blowin' in the Wind|The Times They Are a-Changin'",
  "Michael Jackson": "Thriller|Billie Jean|Beat It",
  Prince: "Purple Rain|When Doves Cry|Kiss",
  Madonna: "Like a Prayer|Vogue|Material Girl",
  "Elvis Presley": "Jailhouse Rock|Hound Dog|Can't Help Falling in Love",
  "Aretha Franklin": "Respect|Think",
  "Stevie Wonder": "Superstition|Sir Duke|Isn't She Lovely",
  "Bob Marley": "No Woman, No Cry|Three Little Birds|One Love",
  Nirvana: "Smells Like Teen Spirit|Come as You Are|Lithium",
  Radiohead: "Creep|Karma Police|Paranoid Android",
  Beyoncé: "Crazy in Love|Halo|Single Ladies",
  "Taylor Swift": "Shake It Off|Blank Space|Love Story",
  Adele: "Rolling in the Deep|Someone Like You|Hello",
  Drake: "Hotline Bling|God's Plan",
  "Kendrick Lamar": "Humble|Alright|Swimming Pools",
  "Billie Eilish": "Bad Guy|Ocean Eyes",
  "Ed Sheeran": "Shape of You|Perfect",
  Rihanna: "Umbrella|Diamonds|We Found Love",
  "Lady Gaga": "Bad Romance|Poker Face",
  "Frank Sinatra": "My Way|Fly Me to the Moon|New York, New York",
  "Louis Armstrong": "What a Wonderful World",
  "Nina Simone": "Feeling Good|I Put a Spell on You",
  "Johnny Cash": "Ring of Fire|I Walk the Line|Folsom Prison Blues",
  "Whitney Houston": "I Wanna Dance with Somebody|Greatest Love of All",
  "Dolly Parton": "Jolene|9 to 5",
  ABBA: "Dancing Queen|Mamma Mia|Waterloo",
  "Fleetwood Mac": "Dreams|Go Your Own Way",
  "Pink Floyd": "Comfortably Numb|Wish You Were Here|Money",
  "Led Zeppelin": "Stairway to Heaven|Kashmir|Whole Lotta Love",
  Eagles: "Hotel California",
  "Ludwig van Beethoven": "Für Elise|Ode to Joy",
  "Wolfgang Amadeus Mozart": "Eine kleine Nachtmusik|Requiem",
  "Johann Sebastian Bach": "Toccata and Fugue in D Minor|Air on the G String",
};

const PAINTINGS: Record<string, string> = {
  "Leonardo da Vinci": "Mona Lisa|The Last Supper",
  "Vincent van Gogh": "The Starry Night|Sunflowers|The Potato Eaters",
  "Pablo Picasso": "Guernica|Les Demoiselles d'Avignon",
  "Claude Monet": "Water Lilies|Impression, Sunrise",
  "Johannes Vermeer": "Girl with a Pearl Earring|The Milkmaid",
  "Edvard Munch": "The Scream",
  "Sandro Botticelli": "The Birth of Venus|Primavera",
  Rembrandt: "The Night Watch",
  "Salvador Dalí": "The Persistence of Memory",
  "Frida Kahlo": "The Two Fridas",
  "Andy Warhol": "Campbell's Soup Cans",
  "Gustav Klimt": "The Kiss",
  "Katsushika Hokusai": "The Great Wave off Kanagawa",
  Michelangelo: "Sistine Chapel Ceiling",
  "Georges Seurat": "A Sunday Afternoon on the Island of La Grande Jatte",
  "Grant Wood": "American Gothic",
  "Edward Hopper": "Nighthawks",
};

const PLAINS: Row[] = [
  plain("activity", "Walking|Reading|Writing|Drawing|Cooking|Baking|Gardening|Fishing|Camping|Knitting|Sewing"),
  plain("activity", "Woodworking|Photography|Birdwatching|Meditation|Yoga|Dancing|Singing|Journaling|Volunteering"),
  plain("activity", "Stargazing|Pottery|Origami|Calligraphy|Napping|Picnicking|Foraging|Beekeeping|Sightseeing|Hiking"),
  plain("sport", "Soccer|Basketball|Baseball|American Football|Tennis|Golf|Cricket|Rugby|Hockey|Volleyball"),
  plain("sport", "Table Tennis|Badminton|Swimming|Running|Cycling|Boxing|Wrestling|Judo|Karate|Fencing|Archery"),
  plain("sport", "Rowing|Sailing|Surfing|Skiing|Snowboarding|Ice Skating|Climbing|Gymnastics|Bowling|Lacrosse|Polo"),
  plain("language", "English|Mandarin|Spanish|Hindi|Arabic|Bengali|Portuguese|Russian|Japanese|German|French"),
  plain("language", "Korean|Italian|Turkish|Vietnamese|Swahili|Dutch|Polish|Greek|Hebrew|Latin|Persian|Urdu|Tamil"),
  plain("language", "Thai|Indonesian|Tagalog|Ukrainian|Swedish|Yoruba|Zulu|Esperanto"),
  plain("occupation", "Teacher|Nurse|Doctor|Farmer|Baker|Chef|Waiter|Barista|Cashier|Carpenter|Plumber|Electrician"),
  plain("occupation", "Mechanic|Driver|Pilot|Sailor|Soldier|Police Officer|Firefighter|Lawyer|Judge|Accountant|Banker"),
  plain("occupation", "Engineer|Programmer|Scientist|Architect|Journalist|Writer|Editor|Translator|Librarian"),
  plain("occupation", "Musician|Painter|Actor|Dancer|Photographer|Designer|Tailor|Barber|Dentist|Pharmacist"),
  plain("occupation", "Veterinarian|Janitor|Gardener|Fisher|Miner|Mail Carrier|Shopkeeper|Professor|Therapist"),
  plain("company", "Apple|Samsung|Toyota|Nike|Coca-Cola|IKEA|Disney|Netflix|Spotify|Walmart|McDonald's|Starbucks"),
  plain("company", "Tesla|Sony|Nintendo|Lego|Visa|Adidas|Boeing"),
  plain("animal", "Dog|Cat|Horse|Cow|Pig|Sheep|Goat|Chicken|Rabbit|Mouse|Elephant|Lion|Tiger|Bear|Wolf|Fox|Deer"),
  plain("animal", "Monkey|Gorilla|Giraffe|Zebra|Kangaroo|Koala|Panda|Penguin|Owl|Eagle|Parrot|Dolphin|Whale|Shark"),
  plain("animal", "Octopus|Frog|Turtle|Snake|Bee|Butterfly|Ant|Llama|Camel"),
  plain("city", "Tokyo|Delhi|Shanghai|São Paulo|Mexico City|Cairo|Mumbai|Beijing|Istanbul|Lagos|Buenos Aires|Moscow"),
  plain("city", "Paris|London|New York|Los Angeles|Chicago|Toronto|Sydney|Rome|Berlin|Madrid|Seoul|Bangkok"),
  plain("city", "Nairobi|Cape Town|Dublin|Athens|Vienna|Amsterdam"),
  plain("country", "Japan|India|China|Brazil|Mexico|Egypt|Turkey|Nigeria|Argentina|Russia|France|United Kingdom"),
  plain("country", "United States|Canada|Australia|Italy|Germany|Spain|South Korea|Thailand|Kenya|South Africa"),
  plain("country", "Ireland|Greece|Austria|Netherlands|Norway|Sweden|Indonesia|Vietnam|Peru|Morocco|Ethiopia"),
  plain("country", "New Zealand"),
  plain("holiday", "New Year|Valentine's Day|Easter|Halloween|Christmas|Thanksgiving|Diwali|Lunar New Year"),
  plain("holiday", "Eid al-Fitr|Hanukkah|Passover|Nowruz|Carnival|Labor Day|Mother's Day|Father's Day"),
];

const ROWS: Row[] = [
  ...FOODS,
  ...GOODS,
  ...works("model", "company", MODELS),
  ...works("book", "author", BOOKS),
  ...works("film", "artist", FILMS),
  ...works("show", "artist", SHOWS),
  ...works("song", "artist", SONGS),
  ...works("painting", "artist", PAINTINGS),
  ...PLAINS,
];

export const slug = (name: string): string =>
  name
    .normalize("NFKD")
    .replace(/\p{M}/gu, "")
    .toLowerCase()
    .replace(/['’]/g, "")
    .replace(/[^\p{L}\p{N}]+/gu, "-")
    .replace(/^-+|-+$/g, "");

function built(rows: readonly Row[]): Thing[] {
  const made = new Map<string, Thing>();
  const place = (name: string, kind: string, more: Partial<Thing> = {}): Thing => {
    const base = slug(name);
    const id = !made.has(base) || made.get(base)!.kind === kind ? base : `${base}-${kind}`;
    const one = made.get(id) ?? { id, name, kind, ...more };
    made.set(id, one);
    return one;
  };
  for (const { kind, names, more, maker } of rows) {
    const by = maker && { by: place(maker[0], maker[1]).id };
    for (const name of names.split("|")) place(name, kind, { ...more, ...by });
  }
  return [...made.values()];
}

export const UNIVERSE: readonly Thing[] = built(ROWS);

export const KINDS: readonly string[] = [...new Set(UNIVERSE.map((one) => one.kind))];

const KNOWN = new Map(UNIVERSE.map((one) => [one.id, one]));

const listed = (value: Json | undefined): string[] | null => (Array.isArray(value) ? value.map(String) : null);

export const kinds = (rules: Obj): string[] => {
  const only = listed(rules[ONLY]);
  return KINDS.filter((kind) => !only || only.includes(kind));
};

const here = (rules: Obj) => {
  const not = new Set(listed(rules[NOT]) ?? []);
  const allowed = new Set(kinds(rules));
  return (one: Thing) => allowed.has(one.kind) && !not.has(one.id) && !(one.by && not.has(one.by));
};

export const things = (rules: Obj, kind?: string): Thing[] =>
  UNIVERSE.filter((one) => !kind || one.kind === kind).filter(here(rules));

export const thing = (rules: Obj, id: string): Thing | null => {
  const one = KNOWN.get(id);
  return one && here(rules)(one) ? one : null;
};

export const known = (id: string): Thing | null => KNOWN.get(id) ?? null;

export const barred = (rules: Obj, id: string): boolean => {
  const not = listed(rules[NOT]) ?? [];
  const by = KNOWN.get(id)?.by;
  return not.includes(id) || (!!by && not.includes(by));
};

export const ruling = (rules: Obj): Obj =>
  Object.fromEntries([NOT, ONLY].filter((key) => rules[key] !== undefined).map((key) => [key, rules[key]!]));

const shuffled = <T>(draw: () => number, list: readonly T[]): T[] => {
  const out = [...list];
  for (let i = out.length - 1; i > 0; i--) {
    const j = below(draw, i + 1);
    [out[i], out[j]] = [out[j]!, out[i]!];
  }
  return out;
};

const ofKind = (kind: string) => UNIVERSE.filter((one) => one.kind === kind);

export const pruned = (draw: () => number, keep: number): string[] => {
  const spared = new Set(
    KINDS.flatMap((kind) => {
      const one = shuffled(draw, ofKind(kind))[0]!;
      return one.by ? [one.id, one.by] : [one.id];
    }),
  );
  const not = new Set<string>();
  for (const kind of KINDS) {
    const all = ofKind(kind);
    const left = all.filter((one) => !one.by || !not.has(one.by));
    let cut = left.length - Math.ceil((all.length * keep) / 100);
    for (const one of shuffled(draw, left)) {
      if (cut > 0 && !spared.has(one.id)) {
        not.add(one.id);
        cut--;
      }
    }
  }
  return [...not];
};

const addedTo = (ctx: Lookup) => (ctx.read(ADDED) ?? {}) as Record<string, Thing>;

const ruled = (ctx: Lookup): Obj =>
  Object.fromEntries(
    [NOT, ONLY].flatMap((key) => {
      const value = ctx.knob<Json>(key);
      return value === null ? [] : [[key, value]];
    }),
  );

export const found = (ctx: Lookup, id: string): Thing | null => {
  const added = addedTo(ctx);
  return thing(ruled(ctx), id) ?? (Object.hasOwn(added, id) ? added[id]! : null);
};

export const existing = (ctx: Lookup, kind?: string): Thing[] => [
  ...things(ruled(ctx), kind),
  ...Object.values(addedTo(ctx)).filter((one) => !kind || one.kind === kind),
];

export const present = (ctx: Lookup, kind: string, id: string): boolean => found(ctx, id)?.kind === kind;

const closest = (word: string, list: readonly string[]): string[] => {
  const left = [...list];
  const out: string[] = [];
  while (out.length < NEAR && left.length) {
    const one = nearest(word, left)!;
    out.push(one);
    left.splice(left.indexOf(one), 1);
  }
  return out;
};

export const absent = (kind: string | null, id: string, ctx: Lookup): string => {
  const what = kind ?? "thing";
  const near = closest(
    slug(id),
    existing(ctx, kind ?? undefined).map((one) => one.id),
  );
  if (!near.length) return `no ${what} exists here; read canon for what does`;
  return `no ${what} ${id} exists here; nearest: ${near.join(", ")}; read canon${kind ? `/${kind}` : ""} for all`;
};
