def create_inventory(items):
    """Create an inventory based on a list.

    :param items: list - list of items to create an inventory from.
    :return:  dict - the inventory dictionary.
    """

    return add_items({}, items)


def add_items(inventory, items):
    """Add items from a list to an existing dictionary.

    :param inventory: dict - dictionary of existing inventory.
    :param items: list - list of items to update the inventory with.
    :return:  dict - the inventory dictionary update with the new items.
    """

    for item in items:
        inventory[item] = inventory.get(item, 0) + 1

    return inventory


def decrement_items(inventory, items):
    """Decrement items from the inventory.

    :param inventory: dict - inventory dictionary.
    :param items: list - list of items to decrement from the inventory.
    :return:  dict - updated inventory dictionary with items decremented.
    """

    for item in items:
        value = inventory.get(item, 0)
        if value > 0:
            value -= 1

        inventory[item] = value

    return inventory


def remove_item(inventory, item):
    """Remove an item entirely from the inventory.

    :param inventory: dict - inventory dictionary.
    :param item: str - item to remove from the inventory.
    :return:  dict - updated inventory dictionary with item removed.
    """

    inventory.pop(item, 0)
    return inventory


def list_inventory(inventory):
    """Return the inventory content.

    :param inventory: dict - an inventory dictionary.
    :return: list of tuples - list of key, value pairs from the inventory dictionary.
    """

    result = []
    for key, value in inventory.items():
        if value == 0:
            continue

        result.append((key, value))

    return result
