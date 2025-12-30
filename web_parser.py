from bs4 import BeautifulSoup


class Stratagem:
    def __init__(self, name: str, arrows: list[str]) -> None:
        self.name: str = name
        self.arrows: list[str] = arrows

    def to_json(self) -> str:
        s = [f"\"{a}\"" for a in self.arrows]
        

        return f"{{\"name\":\"{self.name}\",\"arrows\":[{','.join(s)}]}}"



def parse_content(content: str) -> list[Stratagem]:
    soup = BeautifulSoup(content, 'html.parser')
    rows = soup.find_all('tr')  # First h1 tag
    rows = [x.find_all('td') for x in rows]

    stratagems: list[Stratagem] = []
    for tds in rows:

        try:
            name = tds[1].text.strip()

            images = tds[2].find_all("img")
            arrows = []
            for image in images:
                arrow = image.attrs.get("alt")
                if arrow is None:
                    break;
                arrow = str(arrow)
                arrows.append(arrow)

            s = Stratagem(name, arrows)
            stratagems.append(s)
        except:
            pass

    return stratagems


if __name__ == "__main__":
    with open("./stratagems.html", 'r') as f:
        content = f.read()

    stratagems = parse_content(content)

    with open('stratagems.json', 'w') as f:
        s = [s.name + "\n\t" + ','.join(s.arrows) for s in stratagems]

        f.write('\n'.join(s))
