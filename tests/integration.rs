use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

#[test]
fn test_zero_1() -> Result<(), Box<dyn Error>> {
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-b0"])
        .assert()
        .failure()
        .code(exit);
    Ok(())
}

#[test]
fn test_zero_2() -> Result<(), Box<dyn Error>> {
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-f0-2"])
        .assert()
        .failure()
        .code(exit);
    Ok(())
}

#[test]
fn test_zero_3b() -> Result<(), Box<dyn Error>> {
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-b0-"])
        .assert()
        .failure()
        .code(exit);
    Ok(())
}

#[test]
fn test_zero_3c() -> Result<(), Box<dyn Error>> {
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-c0-"])
        .assert()
        .failure()
        .code(exit);
    Ok(())
}

#[test]
fn test_zero_3f() -> Result<(), Box<dyn Error>> {
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-f0-"])
        .assert()
        .failure()
        .code(exit);
    Ok(())
}

#[test]
fn test_1() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a:c\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f1,3-"])
        .write_stdin("a:b:c\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_2() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a:c\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f1,3-"])
        .write_stdin("a:b:c\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_3() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("b:c\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f2-"])
        .write_stdin("a:b:c\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_4() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f4"])
        .write_stdin("a:b:c\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_5() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f4"])
        .write_stdin("")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_6() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("\n")?;
    Command::cargo_bin("lop")?
        .args(&["-c4"])
        .write_stdin("123\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_7() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("\n")?;
    Command::cargo_bin("lop")?
        .args(&["-c4"])
        .write_stdin("123")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_8() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("\n\n")?;
    Command::cargo_bin("lop")?
        .args(&["-c4"])
        .write_stdin("123\n1")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_9() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-c4"])
        .write_stdin("")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_a() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("c\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f3-"])
        .write_stdin("a:b:c\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_b() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("b:c\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f2,3"])
        .write_stdin("a:b:c\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_c() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a:c\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f1,3"])
        .write_stdin("a:b:c\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_d() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a:c\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f1,3"])
        .write_stdin("a:b:c:\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_e() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("c:\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f3-"])
        .write_stdin("a:b:c:\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_f() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("c:\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f3-4"])
        .write_stdin("a:b:c:\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_g() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("c:\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f3,4"])
        .write_stdin("a:b:c:\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_h() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f2,3"])
        .write_stdin("abc\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_i() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("::\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f1-3"])
        .write_stdin(":::\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_j() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match(":::\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f1-4"])
        .write_stdin(":::\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_k() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match(":\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f2-3"])
        .write_stdin(":::\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_l() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("::\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f2-4"])
        .write_stdin(":::\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_m() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("::\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f1-3"])
        .write_stdin(":::\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_n() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match(":::\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f1-4"])
        .write_stdin(":::\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_o() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match(":\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f2-3"])
        .write_stdin(":::\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_p() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("::\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f2-4"])
        .write_stdin(":::\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_q() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("::\n\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f2-4"])
        .write_stdin(":::\n:\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_r() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("::\n1\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f2-4"])
        .write_stdin(":::\n:1\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_s() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match(":::\n:a\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f1-4"])
        .write_stdin(":::\n:a\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_t() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match(":\n\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f3-"])
        .write_stdin(":::\n:1\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_u() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-f3-"])
        .write_stdin("")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_v() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-f3-"])
        .write_stdin("")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_w() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-b", "1"])
        .write_stdin("")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_x() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f2-4"])
        .write_stdin(":\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_y() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-s", "-b4"])
        .write_stdin(":\n")
        .assert()
        .failure()
        .stdout(out)
        .code(exit);
    Ok(())
}

#[test]
fn test_z() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&[""])
        .write_stdin(":\n")
        .assert()
        .failure()
        .stdout(out)
        .code(exit);
    Ok(())
}

#[test]
fn test_empty_fl() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-f", "''"])
        .write_stdin(":\n")
        .assert()
        .failure()
        .stdout(out)
        .code(exit);
    Ok(())
}

#[test]
fn test_missing_fl() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-f", "--"])
        .write_stdin(":\n")
        .assert()
        .failure()
        .stdout(out)
        .code(exit);
    Ok(())
}

#[test]
fn test_empty_bl() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-b", "''"])
        .write_stdin(":\n")
        .assert()
        .failure()
        .stdout(out)
        .code(exit);
    Ok(())
}

#[test]
fn test_missing_bl() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-b", "--"])
        .write_stdin(":\n")
        .assert()
        .failure()
        .stdout(out)
        .code(exit);
    Ok(())
}

#[test]
fn test_empty_f1() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-f1"])
        .write_stdin("")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_empty_f2() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-f2"])
        .write_stdin("")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_o_delim() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("b:c\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f2,3"])
        .write_stdin("a:b:c\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_nul_idelim() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("b\0c\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d", "", "-f2,3"])
        .write_stdin("a\0b\0c\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_nul_odelim() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("b:c\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f2,3"])
        .write_stdin("a:b:c\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_delim_no_field1() -> Result<(), Box<dyn Error>> {
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-d", "''", "-b1"])
        .assert()
        .failure()
        .code(exit);
    Ok(())
}

#[test]
fn test_delim_no_field2() -> Result<(), Box<dyn Error>> {
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-d:", "-b1"])
        .assert()
        .failure()
        .code(exit);
    Ok(())
}

#[test]
fn test_newline_1() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a\nb\n")?;
    Command::cargo_bin("lop")?
        .args(&["-f1-"])
        .write_stdin("a\nb")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_2() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-f1-"])
        .write_stdin("")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_3() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a\nb\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f1"])
        .write_stdin("a:1\nb:2\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_4() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a\nb\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f1"])
        .write_stdin("a:1\nb:2")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_5() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("1\n2\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f2"])
        .write_stdin("a:1\nb:2\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_6() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("1\n2\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f2"])
        .write_stdin("a:1\nb:2")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_7() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a\nb\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f1"])
        .write_stdin("a:1\nb:2")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_8() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a\nb\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f1"])
        .write_stdin("a:1\nb:2\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_9() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f1"])
        .write_stdin("a1\nb2")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_10() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a:1\nb:2\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f1,2"])
        .write_stdin("a:1\nb:2")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_11() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a:1\nb:2\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f1,2"])
        .write_stdin("a:1\nb:2\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_12() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a\nb\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d:", "-f1"])
        .write_stdin("a:1\nb:")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_13() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a1:\n:\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d:", "-f1-"])
        .write_stdin("a1:\n:")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_14() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a:1\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d", "\n", "-f1"])
        .write_stdin("a:1\nb:")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_15() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("a:1\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d", "\n", "-f1"])
        .write_stdin("a:1\nb:")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_16() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("b\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d", "\n", "-f2"])
        .write_stdin("\nb")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_17() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d", "\n", "-f1"])
        .write_stdin("\nb")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_18() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("b\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d", "\n", "-f2"])
        .write_stdin("\nb")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_19() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d", "\n", "-f1"])
        .write_stdin("\nb")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_20() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d", "\n", "-f1-"])
        .write_stdin("\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_21() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("\nb\n")?;
    Command::cargo_bin("lop")?
        .args(&["-s", "-d", "\n", "-f1-"])
        .write_stdin("\nb")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_newline_22() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("\nb\n")?;
    Command::cargo_bin("lop")?
        .args(&["-d", "\n", "-f1-"])
        .write_stdin("\nb")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_inval1() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-f", "2-0"])
        .write_stdin("")
        .assert()
        .failure()
        .stdout(out)
        .code(exit);
    Ok(())
}

#[test]
fn test_inval2() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-f", "-"])
        .write_stdin("")
        .assert()
        .failure()
        .stdout(out)
        .code(exit);
    Ok(())
}

#[test]
fn test_inval3() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-f", "4,-"])
        .write_stdin("")
        .assert()
        .failure()
        .stdout(out)
        .code(exit);
    Ok(())
}

#[test]
fn test_inval4() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-f", "1-2,-"])
        .write_stdin("")
        .assert()
        .failure()
        .stdout(out)
        .code(exit);
    Ok(())
}

#[test]
fn test_inval5() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-f", "1-,-"])
        .write_stdin("")
        .assert()
        .failure()
        .stdout(out)
        .code(exit);
    Ok(())
}

#[test]
fn test_inval6() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    let exit = predicate::eq(1);
    Command::cargo_bin("lop")?
        .args(&["-f", "-1,-"])
        .write_stdin("")
        .assert()
        .failure()
        .stdout(out)
        .code(exit);
    Ok(())
}

#[test]
fn test_big_unbounded_b() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-b1234567890-"])
        .write_stdin("")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_big_unbounded_b2a() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("19\n")?;
    Command::cargo_bin("lop")?
        .args(&["-b1,9-"])
        .write_stdin("123456789")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_big_unbounded_b2b() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-b1,1234567890-"])
        .write_stdin("")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_big_unbounded_c() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-c1234567890-"])
        .write_stdin("")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_big_unbounded_f() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("")?;
    Command::cargo_bin("lop")?
        .args(&["-f1234567890-"])
        .write_stdin("")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_overlapping_unbounded_1() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("234\n")?;
    Command::cargo_bin("lop")?
        .args(&["-b3-,2-"])
        .write_stdin("1234\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_overlapping_unbounded_2() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("234\n")?;
    Command::cargo_bin("lop")?
        .args(&["-b2-,3-"])
        .write_stdin("1234\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_eol_subsumed_1() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("23456\n")?;
    Command::cargo_bin("lop")?
        .args(&["-b2-,3,4-4,5"])
        .write_stdin("123456\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_eol_subsumed_2() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("23456\n")?;
    Command::cargo_bin("lop")?
        .args(&["-b3,4-4,5,2-"])
        .write_stdin("123456\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}

#[test]
fn test_eol_subsumed_4() -> Result<(), Box<dyn Error>> {
    let out = predicate::str::is_match("1234\n")?;
    Command::cargo_bin("lop")?
        .args(&["-b1-2,2-3,3-"])
        .write_stdin("1234\n")
        .assert()
        .success()
        .stdout(out);
    Ok(())
}
