from sys import argv
from collections import Counter

class Hand:
    def __init__(self, hand, bid):
        self.hand = hand
        self.bid = int(bid)
        self.hand_value = self.get_hand_value()
    
    def get_hand_value(self) -> list[int]:
        counter = Counter((elt for elt in self.hand if elt != "J"))
        j_s = len([elt for elt in self.hand if elt == "J"])
        if j_s == 5:
            return [1000] + [self.card_val(elt) for elt in self.hand]

        top_nums = max(counter.values())
        if top_nums + j_s == 5:
            hand_val = 1000
        elif top_nums + j_s == 4:
            hand_val = 500
        elif top_nums + j_s == 3:
            if j_s:
                if len([elt for elt in counter.values() if elt == 2]) == 2:
                    hand_val = 300
                else: 
                    hand_val = 200
            else:
                if 2 in counter.values():
                    hand_val = 300
                else:
                    hand_val = 200
            
        elif top_nums + j_s == 2:
            if len([elt for elt in counter.values() if elt == 2]) == 2:
                hand_val = 100
            else:
                hand_val = 50
        else:
            hand_val = 0

        return [hand_val] + [self.card_val(elt) for elt in self.hand]

    @classmethod
    def card_val(cls, card: str):
        if card.isdigit():
            return int(card)
        match card:
            case "T":
                return 10
            case "J":
                return 0
            case "Q":
                return 12
            case "K":
                return 13
            case "A":
                return 14
            case _:
                raise ValueError(card)


    def __lt__(self, other: "Hand"):
        return self.hand_value < other.hand_value

if __name__ == "__main__":

    hands = []

    with open(argv[1], "r") as input_:
       for line in input_:
            c_hand, bid = line.rstrip().split()
            hands.append(Hand(hand=c_hand, bid=bid))
            if "J" in line:
                print(hands[-1].hand, hands[-1].hand_value)
    
    total = 0
    for mult, hand in enumerate(sorted(hands)):
        total += (mult+1)*hand.bid

    print(total)