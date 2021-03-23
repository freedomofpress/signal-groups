import pytest

from signal_groups.api.server_params import ServerSecretParams
from signal_groups.api.groups import GroupMasterKey, GroupSecretParams
from signal_groups.api.profiles import ProfileKey
from signal_groups.crypto import ZkGroupException

from tests.constants import TEST_16, TEST_32, TEST_32_1


def test_integration_auth():
    # Server generates parameters
    server_secret_params = ServerSecretParams.generate(TEST_32)
    # Server shares public parameters with clients
    server_public_params = server_secret_params.get_public_params()

    # Client creates group, keeps key and parameters private
    master_key = GroupMasterKey(TEST_32_1)
    group_secret_params = GroupSecretParams.derive_from_master_key(master_key)
    # Public parameters are shared with the server
    group_public_params = group_secret_params.get_public_params()

    uid = TEST_16
    redemption_time = 123456

    # Server issues credential
    randomness = TEST_32
    auth_credential_response = server_secret_params.issue_auth_credential(
        randomness, uid, redemption_time
    )

    # Client gets and verifies auth credential
    auth_credential = server_public_params.receive_auth_credential(
        uid, redemption_time, auth_credential_response
    )

    # Client gets its own uuid entry from the server, decrypts locally.
    uuid_ciphertext = group_secret_params.encrypt_uuid(uid)
    plaintext = group_secret_params.decrypt_uuid(uuid_ciphertext)
    assert plaintext == uid

    # Client creates and presents AuthCredentialPresentation
    randomness = TEST_32
    presentation = server_public_params.create_auth_credential_presentation(
        randomness, group_secret_params, auth_credential
    )

    # Server verifies AuthCredentialPresentation
    # this should not raise Exception
    server_secret_params.verify_auth_credential_presentation(
        group_public_params, presentation
    )


def test_integration_profile():
    # Server generates parameters
    server_secret_params = ServerSecretParams.generate(TEST_32)
    # Server shares public parameters with clients
    server_public_params = server_secret_params.get_public_params()

    # Client creates group, keeps key and parameters private
    master_key = GroupMasterKey(TEST_32_1)
    group_secret_params = GroupSecretParams.derive_from_master_key(master_key)
    # Public parameters are shared with the server
    group_public_params = group_secret_params.get_public_params()

    # Client creates profile key and commitment
    uid = b"dis can be auuid"  # gotta be 16 bytes
    profile_key = ProfileKey.create(TEST_32_1)
    profile_key_commitment = profile_key.get_commitment(uid)

    # Client generates a request
    randomness = TEST_32_1
    context = server_public_params.create_profile_key_credential_request_context(
        randomness, uid, profile_key
    )
    request = context.get_request()

    # Server processes request
    randomness = TEST_32
    response = server_secret_params.issue_profile_key_credential(
        randomness, request, uid, profile_key_commitment
    )

    # Client gets stored profile credential
    profile_key_credential = server_public_params.receive_profile_key_credential(
        context, response
    )

    # Client creates encrypted UID and profile key
    uuid_ciphertext = group_secret_params.encrypt_uuid(uid)
    plaintext = group_secret_params.decrypt_uuid(uuid_ciphertext)
    assert bytes(plaintext) == uid

    profile_key_ciphertext = group_secret_params.encrypt_profile_key(profile_key, uid)
    decrypted_profile_key = group_secret_params.decrypt_profile_key(
        profile_key_ciphertext, uid
    )
    assert decrypted_profile_key.get_bytes() == profile_key.get_bytes()

    # Client creates profile key presentation
    randomness = TEST_32
    presentation = server_public_params.create_profile_key_credential_presentation(
        randomness,
        group_secret_params,
        profile_key_credential,
    )

    # Server verifies profile key presentation
    # If verifies, should not raise exception
    server_secret_params.verify_profile_key_credential_presentation(
        group_public_params, presentation
    )


def test_server_sigs():
    server_secret_params = ServerSecretParams.generate(TEST_32)
    server_public_params = server_secret_params.get_public_params()

    randomness = TEST_32
    message = b"dis can be auuid"
    signature = server_secret_params.sign(randomness, message)

    # Should not raise exception
    signature_1 = signature[:32]
    signature_2 = signature[32:]
    server_public_params.verify_signature(message, signature_1, signature_2)

    message = bytes([message[0] ^ 0x01]) + message[1:]
    with pytest.raises(ZkGroupException, match="SignatureVerificationFailure"):
        server_public_params.verify_signature(message, signature_1, signature_2)


def test_blob_encryption():
    master_key = GroupMasterKey(TEST_32_1)
    group_secret_params = GroupSecretParams.derive_from_master_key(master_key)

    randomness = TEST_32
    message = b"dis can be auuid"

    calc_ciphertext_vec = group_secret_params.encrypt_blob(randomness, message)
    calc_plaintext_vec = group_secret_params.decrypt_blob(calc_ciphertext_vec)

    assert calc_plaintext_vec == message
