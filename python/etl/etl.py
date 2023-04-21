def transform(legacy_data):
    return {
        value.lower(): key
        for key, values in legacy_data.items()
        for value in values
    }
