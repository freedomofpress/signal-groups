from signal_groups.api import server_params

from tests.constants import TEST_16, TEST_32


def test_ServerSecretParams_serialization():
    random_arr = TEST_32
    test_params = server_params.ServerSecretParams.generate(random_arr)

    serialized_obj = test_params.serialize()
    deserialized_obj = server_params.ServerSecretParams.deserialize(serialized_obj)

    assert serialized_obj == deserialized_obj.serialize()


def test_ServerPublicParams_serialization():
    random_arr = TEST_32
    test_secret_params = server_params.ServerSecretParams.generate(random_arr)
    test_public_params = test_secret_params.get_public_params()

    serialized_obj = test_public_params.serialize()
    deserialized_obj = server_params.ServerPublicParams.deserialize(serialized_obj)

    assert serialized_obj == deserialized_obj.serialize()
