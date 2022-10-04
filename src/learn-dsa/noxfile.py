import nox


@nox.session(python=["3.9"])
def lint(session):
    session.install("mypy==0.910")
    session.install("pylint==2.11.1")

    session.run("mypy", "src")
    session.run("pylint", "src")


@nox.session(python=["3.9", "3.10"])
def test(session):
    session.install("pytest==6.2.5")
    session.run("pytest", "src", "-v")


@nox.session(python=["3.9"])
def coverage(session):
    session.install("pytest==6.2.5")
    session.install("coverage==6.1.2")

    session.run("coverage", "run", "-m", "pytest", "src")
    session.run("coverage", "report", "-m")
