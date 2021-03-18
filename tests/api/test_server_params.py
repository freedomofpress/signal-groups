from signal_groups.api import server_params


def test_ServerSecretParams_serialization():
    random_arr = [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
        28,
        29,
        30,
        31,
    ]
    test_params = server_params.ServerSecretParams.generate(random_arr)

    serialized_obj = test_params.serialize()
    deserialized_obj = server_params.ServerSecretParams.deserialize(serialized_obj)

    assert serialized_obj == deserialized_obj.serialize()


def test_ServerPublicParams_serialization():
    random_arr = [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
        28,
        29,
        30,
        31,
    ]
    test_secret_params = server_params.ServerSecretParams.generate(random_arr)
    test_public_params = test_secret_params.get_public_params()

    serialized_obj = test_public_params.serialize()
    deserialized_obj = server_params.ServerPublicParams.deserialize(serialized_obj)

    assert serialized_obj == deserialized_obj.serialize()
