import hmac, hashlib

def generate_mac(nonce, user, password, admin=False, user_type=None):

    mac = hmac.new(
      key=b"m@;wYOUOh0f:CH5XA65sJB1^q01~DmIriOysRImot,OR_vzN&B",
      digestmod=hashlib.sha1,
    )

    mac.update(nonce.encode('utf8'))
    mac.update(b"\x00")
    mac.update(user.encode('utf8'))
    mac.update(b"\x00")
    mac.update(password.encode('utf8'))
    mac.update(b"\x00")
    mac.update(b"admin" if admin else b"notadmin")
    if user_type:
        mac.update(b"\x00")
        mac.update(user_type.encode('utf8'))

    return mac.hexdigest()

if __name__ == "__main__":
  mac = generate_mac(
    nonce="1234567890",
    user="groot",
    password="imroot!1234",
    admin=True,
    user_type=None
  )

  print(mac)
